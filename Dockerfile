# ThreatFlux Rust Dockerfile
# Multi-stage build for the OpenAI Rust SDK.

FROM rust:1.96.0-bookworm AS rust-base

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

FROM debian:bookworm-slim AS runtime

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

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    tini \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /data /config /output /usr/share/doc/openai-rust-sdk \
    && useradd -m -u 1000 -s /bin/bash openai

COPY --from=builder /build/target/release/${BINARY_NAME} /usr/local/bin/${CLI_NAME}
COPY --from=builder /build/${BINARY_NAME}-sbom.json /usr/share/doc/openai-rust-sdk/sbom.cdx.json
COPY --chown=openai:openai test_data /opt/openai-rust-sdk/test_data

RUN chown -R openai:openai \
    /data \
    /config \
    /output \
    /opt/openai-rust-sdk \
    /usr/local/bin/${CLI_NAME} \
    /usr/share/doc/openai-rust-sdk

USER openai
WORKDIR /data

ENV RUST_LOG=info
ENV OPENAI_BASE_URL=https://api.openai.com/v1

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/openai-rust-sdk", "--version"] || exit 1

EXPOSE 3000 8080

ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/openai-rust-sdk"]
