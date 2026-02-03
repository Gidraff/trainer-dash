FROM rust:1.91 as builder

WORKDIR /app
# Install build dependencies for crates like openssl or sqlx
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY .sqlx ./.sqlx

COPY . .

ENV SQLX_OFFLINE=true
# Final build of your actual source code
RUN cargo build --release

FROM debian:bookworm-slim
# CRITICAL: Install SSL libraries in the runtime image
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Ensure 'api' matches the name in your Cargo.toml [package] section
COPY --from=builder /app/target/release/api ./api

EXPOSE 8080
# Use the correct binary name
CMD ["./api"]