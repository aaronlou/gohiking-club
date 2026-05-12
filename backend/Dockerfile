# ── Build stage ──
FROM rust:1.94-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release 2>/dev/null || true

# Build the actual application
COPY . .
RUN touch src/main.rs && cargo build --release

# ── Runtime stage ──
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and migrations
COPY --from=builder /app/target/release/gohiking-backend /app/gohiking-backend
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/config /app/config

# Create uploads directory for local storage
RUN mkdir -p /app/uploads

ENV RUST_LOG=info
EXPOSE 3000

ENTRYPOINT ["./gohiking-backend"]
