# Builder stage
FROM rust:1.91-slim-bookworm as builder

WORKDIR /usr/src/app

# Install build dependencies (libpq for diesel)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies first (caching layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy build artifacts
RUN rm -f target/release/deps/nuggetsync*

# Copy actual source code
COPY src ./src
COPY migrations ./migrations

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/nuggetsync /app/nuggetsync

# Expose the application port
EXPOSE 3001

# Run the application
CMD ["/app/nuggetsync"]
