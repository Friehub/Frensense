# --- Build Stage ---
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /usr/src/frensense
COPY . .

# Build with optimizations — produces both frensense and frensense-mcp binaries
RUN cargo build --release

# --- Final Stage ---
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/frensense/target/release/frensense /usr/local/bin/frensense

# Entrypoint
ENTRYPOINT ["frensense"]
CMD ["--help"]
