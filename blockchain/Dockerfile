# Build stage
FROM rust:1.70 as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release --bin optimic-node

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r optimic && useradd -r -g optimic optimic

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/optimic-node /usr/local/bin/optimic-node

# Copy configuration files
COPY config.toml genesis.json ./

# Create data directory
RUN mkdir -p /app/data && chown -R optimic:optimic /app

# Switch to app user
USER optimic

# Expose ports
EXPOSE 26656 26657

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:26657/health || exit 1

# Run the application
ENTRYPOINT ["optimic-node"]
CMD ["start", "--config", "/app/config.toml", "--genesis", "/app/genesis.json"]
