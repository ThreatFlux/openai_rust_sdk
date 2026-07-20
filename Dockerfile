# ThreatFlux Rust Dockerfile
# Multi-stage build for the OpenAI Rust SDK.

# Base images are pinned by digest for reproducibility (Scorecard Pinned-Dependencies).
# Refresh with: docker buildx imagetools inspect <image> | awk '/^Digest:/{print $2}'
FROM rust:1.97.0-bookworm@sha256:7d0723df719e7f213b69dc7c8c595985c3f4b060cfbee4f7bc0e347a86fe3b6a AS rust-base

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG BINARY_NAME=openai_rust_sdk
ARG BINARY_PACKAGE=
ARG CLI_NAME=openai-rust-sdk
ARG SBOM_MANIFEST_PATH=Cargo.toml
ARG OCI_IMAGE_TITLE=OpenAI Rust SDK
ARG OCI_IMAGE_DESCRIPTION=Comprehensive OpenAI API SDK for Rust with YARA rule validation
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/threatflux/openai_rust_sdk

RUN apt-get update && apt-get install -y \
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

RUN if [ -n "${BINARY_PACKAGE}" ]; then \
      cargo build --release -p "${BINARY_PACKAGE}" --bin "${BINARY_NAME}" --all-features; \
    else \
      cargo build --release --bin "${BINARY_NAME}" --all-features || cargo build --release --all-features; \
    fi

RUN cargo install cargo-cyclonedx --locked --version 0.5.8 && \
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
# Pinned by digest (Scorecard Pinned-Dependencies).
FROM gcr.io/distroless/cc-debian12:nonroot@sha256:ce0d66bc0f64aae46e6a03add867b07f42cc7b8799c949c2e898057b7f75a151 AS runtime

ARG VERSION=0.0.0
ARG BUILD_DATE=unknown
ARG VCS_REF=unknown
ARG BINARY_NAME=openai_rust_sdk
ARG CLI_NAME=openai-rust-sdk
ARG OCI_IMAGE_TITLE=OpenAI Rust SDK
ARG OCI_IMAGE_DESCRIPTION=Comprehensive OpenAI API SDK for Rust with YARA rule validation
ARG OCI_IMAGE_VENDOR=ThreatFlux
ARG OCI_IMAGE_SOURCE=https://github.com/threatflux/openai_rust_sdk

LABEL org.opencontainers.image.title="${OCI_IMAGE_TITLE}" \
      org.opencontainers.image.description="${OCI_IMAGE_DESCRIPTION}" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.vendor="${OCI_IMAGE_VENDOR}" \
      org.opencontainers.image.source="${OCI_IMAGE_SOURCE}" \
      org.opencontainers.image.authors="Wyatt Roersma <wyattroersma@gmail.com>, Claude Code" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.documentation="https://github.com/threatflux/openai_rust_sdk/blob/main/README.md" \
      com.threatflux.category="AI/ML SDK" \
      com.threatflux.capabilities="openai,batch-processing,yara-validation" \
      com.threatflux.rust.version="1.96.0" \
      com.threatflux.rust.edition="2024"

# tini is copied from the build stage for proper PID 1 signal handling/zombie
# reaping (distroless has no init).
COPY --from=builder /usr/bin/tini /usr/bin/tini

COPY --from=builder /build/target/release/${BINARY_NAME} /usr/local/bin/${CLI_NAME}
COPY --from=builder /build/${BINARY_NAME}-sbom.json /usr/share/doc/openai-rust-sdk/sbom.cdx.json
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
