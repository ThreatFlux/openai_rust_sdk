# ThreatFlux Rust Dockerfile
# Multi-stage build for single-crate or workspace-based applications.
#
# Build a fully static musl CLI and tini, then ship only those executables,
# CA certificates and nonroot account data in distroless/static. No glibc,
# zlib, GCC shared runtime, shell or package manager is shipped. Keep all
# Cargo features: native TLS uses statically vendored OpenSSL; rustls remains
# available too. Runtime trust roots are explicit for both TLS backends.
# Repo-specific values are build ARGs; .github/workflows/docker.yml supplies
# them from repository variables.
#
# Base images are pinned by digest for reproducibility (Scorecard
# Pinned-Dependencies). Refresh with:
#   docker buildx imagetools inspect <image> | awk '/^Digest:/{print $2}'
# Dependabot refreshes the Rust builder; refresh the runtime digest with the
# command above when it is updated.

# Refresh trust data and its package provenance together. Distroless currently
# has fixed CA data but stale package metadata; never globally dismiss the
# advisory while older published images still trust the distrusted roots.
FROM --platform=$BUILDPLATFORM debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad AS trust-store

# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    dpkg --compare-versions "$(dpkg-query -W -f='${Version}' ca-certificates)" ge 20250419+deb13u1 && \
    dpkg-query -s ca-certificates > /ca-certificates.status && \
    rm -rf /var/lib/apt/lists/*

# rust 1.99.1 on Alpine 3.24 (musl); multi-arch index digest
FROM rust:1.99.1-alpine3.24@sha256:557a94c82b485a14b569120e00867267108c734efe33f10cc31ca0cd5dd1eca9 AS rust-base

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG BINARY_NAME=openai_rust_sdk
ARG BINARY_PACKAGE=openai_rust_sdk
ARG CLI_NAME=openai-rust-sdk
ARG SBOM_MANIFEST_PATH=Cargo.toml
ARG OCI_IMAGE_TITLE="OpenAI Rust SDK"
ARG OCI_IMAGE_DESCRIPTION="Batch JSONL generation and local YARA-X validation CLI"
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/ThreatFlux/openai_rust_sdk

# tini-static preserves signal forwarding and zombie reaping without a libc
# dependency. Build tools stay in the builder; OpenSSL is vendored by Cargo.
RUN apk add --no-cache \
    binutils \
    build-base \
    ca-certificates \
    cmake \
    linux-headers \
    perl \
    pkgconf \
    tini-static

FROM rust-base AS builder

RUN adduser -D -u 1000 builder
USER builder
WORKDIR /build

ENV CARGO_HOME=/home/builder/.cargo
ENV PATH="/home/builder/.cargo/bin:${PATH}"

# Only build inputs invalidate native compilation; trust-store and validation
# changes can refresh the runtime without rebuilding unchanged Rust code.
COPY --chown=builder:builder Cargo.toml Cargo.lock rust-toolchain.toml README.md LICENSE ./
COPY --chown=builder:builder .cargo/ .cargo/
COPY --chown=builder:builder src/ src/
COPY --chown=builder:builder examples/ examples/
COPY --chown=builder:builder tests/ tests/
COPY --chown=builder:builder benches/ benches/
COPY --chown=builder:builder test_data/ test_data/

# An explicit target keeps host proc macros separate from target static flags.
# Disable Rust's older bundled musl CRT: link the maintained Alpine libc.a.
# Keep only Rust's LLVM unwinder in a separate search path (never bundled libc).
# Prove the selected archive from the link map and reject dynamic dependencies.
RUN rustc --version --verbose && cargo --version && \
    build_target="$(rustc -vV | sed -n 's/^host: //p')" && \
    mkdir -p /home/builder/native-unwind && \
    cp "$(rustc --print sysroot)/lib/rustlib/${build_target}/lib/self-contained/libunwind.a" /home/builder/native-unwind/ && \
    RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=no -C linker=cc -C relocation-model=pic -C link-arg=-static-pie -L native=/home/builder/native-unwind -C link-arg=-Wl,-Map=/home/builder/native-link.map" OPENSSL_STATIC=1 \
      cargo build --locked --release --target "${build_target}" \
        ${BINARY_PACKAGE:+-p "${BINARY_PACKAGE}"} --bin "${BINARY_NAME}" --all-features && \
    musl_archive="$(grep -oE '/[^[:space:](]+/libc\.a' /home/builder/native-link.map | sort -u)" && \
    test "$(readlink -f "${musl_archive}")" = /usr/lib/libc.a && \
    readelf -h "target/${build_target}/release/${BINARY_NAME}" | grep -q 'Type:.*DYN' && \
    for executable in "target/${build_target}/release/${BINARY_NAME}" /sbin/tini-static; do \
      if readelf -l "${executable}" | grep -q INTERP || \
         readelf -d "${executable}" | grep -q NEEDED; then \
        echo "Dynamic dependency in ${executable}" >&2; exit 1; \
      fi; \
    done

# cargo-cyclonedx writes the SBOM beside the manifest it was handed, which in a
# workspace is not necessarily /build. The find normalizes the output location.
RUN cargo install cargo-cyclonedx --locked --version 0.5.9 && \
    cargo cyclonedx \
      --manifest-path "${SBOM_MANIFEST_PATH}" \
      --all-features \
      --format json \
      --spec-version 1.5 \
      --override-filename "${BINARY_NAME}-sbom" && \
    find /build -name "${BINARY_NAME}-sbom.json" -exec cp {} /build/sbom.cdx.json \; -quit && \
    test -s /build/sbom.cdx.json

# Stage the runtime filesystem under a fixed layout.
#
# The binary is staged as `app` at a FIXED path: exec-form ENTRYPOINT and
# HEALTHCHECK do not expand build ARGs. A symlink keeps the CLI's own name
# (`vertex`) available on PATH.
#
# Writable directories are created here because the distroless runtime has no
# shell to mkdir with; ownership is applied on the way in via COPY --chown.
RUN mkdir -p /home/builder/out/bin /home/builder/out/doc \
             /home/builder/runtime-skel/data \
             /home/builder/runtime-skel/config \
             /home/builder/runtime-skel/output && \
    build_target="$(rustc -vV | sed -n 's/^host: //p')" && \
    cp "target/${build_target}/release/${BINARY_NAME}" /home/builder/out/bin/app && \
    if [ "${CLI_NAME}" != "app" ]; then \
      ln -s app "/home/builder/out/bin/${CLI_NAME}"; \
    fi && \
    cp /build/sbom.cdx.json /home/builder/out/doc/sbom.cdx.json && \
    { rustc -vV; apk list --installed musl musl-dev tini-static; \
      sha256sum /usr/lib/libc.a /sbin/tini-static /home/builder/native-unwind/libunwind.a; \
      grep -oE '/[^[:space:](]+/libc\.a' /home/builder/native-link.map | sort -u; \
    } > /home/builder/out/doc/native-build.txt

# Static distroless: certificates/account data, no dynamic libc; multi-arch digest
FROM gcr.io/distroless/static-debian13:nonroot@sha256:516757106de35205efb70b91ba03f6f1298424a0678c603255cf2852a7000087 AS runtime

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG OCI_IMAGE_TITLE="OpenAI Rust SDK"
ARG OCI_IMAGE_DESCRIPTION="Batch JSONL generation and local YARA-X validation CLI"
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/ThreatFlux/openai_rust_sdk
ARG OCI_IMAGE_LICENSES=MIT
ARG OCI_IMAGE_DOCUMENTATION=https://github.com/ThreatFlux/openai_rust_sdk/blob/main/README.md

LABEL org.opencontainers.image.title="${OCI_IMAGE_TITLE}" \
      org.opencontainers.image.description="${OCI_IMAGE_DESCRIPTION}" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.vendor="${OCI_IMAGE_VENDOR}" \
      org.opencontainers.image.source="${OCI_IMAGE_SOURCE}" \
      org.opencontainers.image.licenses="${OCI_IMAGE_LICENSES}" \
      org.opencontainers.image.documentation="${OCI_IMAGE_DOCUMENTATION}" \
      com.threatflux.rust.version="1.99.1" \
      com.threatflux.rust.edition="2024"

# These are the only CA trust files in the pinned static base. Copy the fixed
# supplier data and exact installed-package record, not a relabeled version.
COPY --from=trust-store /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=trust-store /ca-certificates.status /var/lib/dpkg/status.d/ca-certificates

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

COPY --from=builder /sbin/tini-static /usr/bin/tini

# The binary and SBOM stay root-owned so the runtime user cannot modify them;
# only the working directories belong to the nonroot user.
COPY --from=builder --chown=0:0 /home/builder/out/bin/ /usr/local/bin/
COPY --from=builder --chown=0:0 /home/builder/out/doc/ /usr/share/doc/openai-rust-sdk/
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/data /data
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/config /config
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/output /output

COPY --chown=0:0 test_data /opt/openai-rust-sdk/test_data

USER 65532:65532
WORKDIR /data

# Exec form (there is no shell in distroless); a nonzero exit means unhealthy.
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/app", "--version"]

# Arguments go straight to the CLI: `docker run <image> --help`.
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/app"]
