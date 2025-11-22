FROM rust:1.85-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo files
COPY Cargo.toml ./

# Remove existing Cargo.lock
RUN rm -f Cargo.lock

# Create dummy main.rs and build dependencies only
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn main() {}" > src/lib.rs && \
    cargo update && \
    cargo build --release && \
    rm -f src/*.rs

# Copy actual source code (but don't overwrite Cargo.lock)
COPY --chown=root:root . .
RUN rm -f Cargo.lock

# Force cargo to use the pinned version and build the application
RUN cargo update && cargo build --release --verbose

# Runtime stage
FROM debian:12-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary and migrations
COPY --from=builder /app/target/release/money-manager-api /app/money-manager-api
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/sample.env /app/.env

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["/app/money-manager-api"]
