# ─── Stage 1: Builder ────────────────────────────────────────────────────────
FROM rust:1.89-slim-bookworm AS builder

WORKDIR /app

# Install dependencies build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies 
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/backend*

# Copy source dan build
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# ─── Stage 2: Runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary dari builder
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/migrations ./migrations

# Non-root user
RUN useradd -r -s /bin/false appuser
USER appuser

EXPOSE 8080

CMD ["./backend"]
