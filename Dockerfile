# --- Build Stage ---
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /usr/src/gensense
COPY . .

# Build with optimizations
RUN cargo build --release --features full

# --- Final Stage ---
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/gensense/target/release/gensense /usr/local/bin/gensense

# Entrypoint
ENTRYPOINT ["gensense"]
CMD ["--help"]
