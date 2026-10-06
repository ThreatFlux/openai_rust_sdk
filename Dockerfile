# ThreatFlux Rust Dockerfile
# Multi-stage build for the OpenAI Rust SDK CLI.
#
# Follows ThreatFlux/rust-cicd-template's canonical layout: a Debian 13
# (trixie) Rust builder and a distroless Debian 13 runtime with no shell,
# package manager or coreutils.
#
# Base images are pinned by digest for reproducibility (Scorecard Pinned-Dependencies).
# Refresh with: docker buildx imagetools inspect <image> | awk '/^Digest:/{print $2}'
# Dependabot refreshes the Rust builder; refresh the runtime digest with the
# command above when it is updated.

# rust 1.99.0 on Debian 13 (trixie); multi-arch index digest
FROM rust:1.99.0-trixie@sha256:15ad267e7a4cb2dce5905c90c76765adb6714945c5ea6d7c82673897a5e4067b AS rust-base

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG BINARY_NAME=openai_rust_sdk
ARG BINARY_PACKAGE=
ARG CLI_NAME=openai-rust-sdk
ARG SBOM_MANIFEST_PATH=Cargo.toml
ARG OCI_IMAGE_TITLE=OpenAI Rust SDK
ARG OCI_IMAGE_DESCRIPTION=Batch JSONL generation and local YARA-X validation CLI
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/threatflux/openai_rust_sdk

# tini is installed here so the runtime stage can copy it out: distroless ships
# no init, and PID 1 must reap zombies and forward signals. Package revisions
# follow the pinned base image's Debian 13 repositories.
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    pkg-config \
    libssl-dev \
    tini \
    && rm -rf /var/lib/apt/lists/*

FROM rust-base AS builder

RUN useradd -m -u 1000 builder
USER builder
WORKDIR /build

ENV CARGO_HOME=/home/builder/.cargo
ENV PATH="/home/builder/.cargo/bin:${PATH}"

COPY --chown=builder:builder . .

RUN rustc --version --verbose && cargo --version && \
    if [ -n "${BINARY_PACKAGE}" ]; then \
      cargo build --locked --release -p "${BINARY_PACKAGE}" --bin "${BINARY_NAME}" --all-features; \
    else \
      cargo build --locked --release --bin "${BINARY_NAME}" --all-features; \
    fi

RUN cargo install cargo-cyclonedx --locked --version 0.5.9 && \
    cargo cyclonedx \
      --manifest-path "${SBOM_MANIFEST_PATH}" \
      --all-features \
      --format json \
      --spec-version 1.5 \
      --override-filename "${BINARY_NAME}-sbom"

# Stage writable runtime directories. The distroless runtime has no shell or
# package manager, so directory creation/ownership is prepared here and the
# ownership is applied via `COPY --chown` into the final image.
RUN mkdir -p /home/builder/runtime-skel/data \
             /home/builder/runtime-skel/config \
             /home/builder/runtime-skel/output

# Distroless runtime: glibc + libssl + libgcc only — no shell, package manager,
# perl, coreutils, tar, passwd, etc. This removes the overwhelming majority of
# unfixable Debian base-image CVEs reported by Trivy. reqwest uses rustls, so no
# OpenSSL is required for HTTP; the `cc` variant still provides libssl for any
# transitive -sys linkage. Runs as the built-in nonroot user (uid 65532).
# distroless cc on Debian 13, nonroot tag; multi-arch index digest
# (Scorecard Pinned-Dependencies).
FROM gcr.io/distroless/cc-debian13:nonroot@sha256:e792ab3d241a468a4fd7519ddbbebe66b49b5f365771716ea688ad40b6c6f1c2 AS runtime

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG BINARY_NAME=openai_rust_sdk
ARG CLI_NAME=openai-rust-sdk
ARG OCI_IMAGE_TITLE=OpenAI Rust SDK
ARG OCI_IMAGE_DESCRIPTION=Batch JSONL generation and local YARA-X validation CLI
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/threatflux/openai_rust_sdk

LABEL org.opencontainers.image.title="${OCI_IMAGE_TITLE}" \
      org.opencontainers.image.description="${OCI_IMAGE_DESCRIPTION}" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.vendor="${OCI_IMAGE_VENDOR}" \
      org.opencontainers.image.source="${OCI_IMAGE_SOURCE}" \
      org.opencontainers.image.authors="ThreatFlux" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.documentation="https://github.com/threatflux/openai_rust_sdk/blob/main/README.md" \
      com.threatflux.category="AI/ML SDK" \
      com.threatflux.capabilities="openai,batch-jsonl-generation,yara-x-validation" \
      com.threatflux.rust.version="1.99.0" \
      com.threatflux.rust.edition="2024"

# tini is copied from the build stage for proper PID 1 signal handling/zombie
# reaping (distroless has no init).
COPY --from=builder /usr/bin/tini /usr/bin/tini

# The binary and SBOM stay root-owned so the runtime user cannot modify them;
# only the working directories belong to the nonroot user.
COPY --from=builder --chown=0:0 /build/target/release/${BINARY_NAME} /usr/local/bin/${CLI_NAME}
COPY --from=builder --chown=0:0 /build/${BINARY_NAME}-sbom.json /usr/share/doc/openai-rust-sdk/sbom.cdx.json
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/data /data
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/config /config
COPY --from=builder --chown=65532:65532 /home/builder/runtime-skel/output /output
COPY --chown=65532:65532 test_data /opt/openai-rust-sdk/test_data

USER 65532:65532
WORKDIR /data

ENV RUST_LOG=info
ENV OPENAI_BASE_URL=https://api.openai.com/v1

# Exec form (no shell in distroless); a nonzero exit is reported as unhealthy.
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/openai-rust-sdk", "--version"]

EXPOSE 3000 8080

ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/openai-rust-sdk"]
