# Build stage
FROM rust:1.89.0-slim AS builder

# Build arguments
ARG VERSION=unknown
ARG BUILD_DATE
ARG VCS_REF

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/openai-rust-sdk

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy directories and files for dependencies
RUN mkdir -p src/api src/models src/testing src/builders src/schema benches examples tests && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib() {}" > src/lib.rs && \
    echo "fn main() {}" > benches/validation_benchmarks.rs && \
    echo "fn main() {}" > benches/json_parsing_benchmarks.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && \
    rm -rf src benches examples tests \
    target/release/deps/*openai*rust*sdk* \
    target/release/.fingerprint/*openai*rust*sdk*

# Copy source code
COPY src ./src
COPY benches ./benches
COPY examples ./examples
COPY tests ./tests

# Build for release with version info and all features
ENV CARGO_PKG_VERSION=${VERSION}
RUN cargo build --release --all-features

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash openai

# Copy the binary from builder
COPY --from=builder /usr/src/openai-rust-sdk/target/release/openai_rust_sdk /usr/local/bin/openai-rust-sdk

# Copy library files for potential linking
COPY --from=builder /usr/src/openai-rust-sdk/target/release/libopenai_rust_sdk.rlib /usr/local/lib/
COPY --from=builder /usr/src/openai-rust-sdk/target/release/deps /usr/local/lib/deps

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
LABEL com.threatflux.rust.version="1.89.0"
LABEL com.threatflux.rust.edition="2021"