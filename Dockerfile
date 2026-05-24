# Dependency cache stage using the ThreatFlux Rust template image
FROM docker.io/threatflux/rust-cicd-template:base-rust-latest@sha256:9c47317b7d35889f1a8b2c8f94c1ec1e0f9b1c0c661380f79fe761ed824c215b AS chef
ARG RUST_TOOLCHAIN=stable
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH
WORKDIR /app
USER root

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL --proto '=https' --tlsv1.2 https://sh.rustup.rs -o /tmp/rustup.sh \
    && echo "6c30b75a75b28a96fd913a037c8581b580080b6ee9b8169a3c0feb1af7fe8caf  /tmp/rustup.sh" | sha256sum -c - \
    && sh /tmp/rustup.sh -y --no-modify-path --default-toolchain ${RUST_TOOLCHAIN} --profile minimal \
    && rm -f /tmp/rustup.sh
RUN cargo install cargo-chef

# Plan the build (generate recipe.json)
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches
COPY examples ./examples
COPY tests ./tests
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies (cached layer)
FROM chef AS builder

# Build arguments
ARG VERSION=unknown
ARG BUILD_DATE
ARG VCS_REF

# Install build dependencies
USER root
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy the build recipe and build dependencies
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching layer!
RUN cargo chef cook --release --recipe-path recipe.json --all-features

# Copy all source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches
COPY examples ./examples
COPY tests ./tests

# Build the application
ENV CARGO_PKG_VERSION=${VERSION}
RUN cargo build --release --all-features

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
USER root
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash openai

# Copy the binary from builder
COPY --from=builder /app/target/release/openai_rust_sdk /usr/local/bin/openai-rust-sdk

# Copy library files for potential linking
COPY --from=builder /app/target/release/libopenai_rust_sdk.rlib /usr/local/lib/
COPY --from=builder /app/target/release/deps /usr/local/lib/deps

# Create necessary directories
RUN mkdir -p /data /config /output && \
    chown -R openai:openai /data /config /output /usr/local/bin/openai-rust-sdk

# Copy test data for validation (optional, remove in production)
COPY --chown=openai:openai test_data /opt/openai-rust-sdk/test_data

# Switch to non-root user
USER openai

# Set working directory
WORKDIR /data

# Environment variables
ENV RUST_LOG=info
ENV OPENAI_BASE_URL=https://api.openai.com/v1

# Default command (can be overridden)
ENTRYPOINT ["openai-rust-sdk"]

# Health check for API connectivity
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["openai-rust-sdk", "--version"] || exit 1

# Expose common API server ports (if SDK includes a server mode)
EXPOSE 3000 8080

# Labels with build info
LABEL org.opencontainers.image.title="OpenAI Rust SDK"
LABEL org.opencontainers.image.description="Comprehensive OpenAI API SDK for Rust with YARA rule validation"
LABEL org.opencontainers.image.authors="Wyatt Roersma <wyattroersma@gmail.com>, Claude Code"
LABEL org.opencontainers.image.source="https://github.com/threatflux/openai_rust_sdk"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.created="${BUILD_DATE}"
LABEL org.opencontainers.image.revision="${VCS_REF}"
LABEL org.opencontainers.image.vendor="ThreatFlux"
LABEL org.opencontainers.image.documentation="https://github.com/threatflux/openai_rust_sdk/blob/main/README.md"

# Additional ThreatFlux-specific labels
LABEL com.threatflux.category="AI/ML SDK"
LABEL com.threatflux.capabilities="openai,batch-processing,yara-validation"
LABEL com.threatflux.rust.version="1.95.0"
LABEL com.threatflux.rust.edition="2024"
