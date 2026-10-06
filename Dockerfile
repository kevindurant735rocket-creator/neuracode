# NeuraCode Dockerfile
# Multi-stage build for optimized production image

# Stage 1: Build Rust binary
FROM rust:1.70-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release --bin neuracode

# Stage 2: Build Python module
FROM python:3.11-slim as python-builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy Python files
COPY python ./python

# Build Python module
RUN cd python && pip install -e .

# Stage 3: Final image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust binary
COPY --from=builder /app/target/release/neuracode /usr/local/bin/neuracode

# Copy Python module
COPY --from=python-builder /usr/local/lib/python3.11/site-packages /usr/local/lib/python3.11/site-packages

# Create non-root user
RUN useradd -m -u 1000 neuracode
USER neuracode

# Set environment variables
ENV RUST_LOG=info
ENV NEURACODE_HOME=/home/neuracode/.neuracode

# Create data directory
RUN mkdir -p $NEURACODE_HOME && chown -R neuracode:neuracode $NEURACODE_HOME

VOLUME ["/home/neuracode/.neuracode"]

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD neuracode version || exit 1

ENTRYPOINT ["neuracode"]
CMD ["--help"]
