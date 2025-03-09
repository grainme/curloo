FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# dummy main.rs to build dependencies first (for better layer caching)
RUN mkdir -p src && \
    echo "fn main() {println!(\"Dummy build\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/curloo /usr/local/bin/curloo

ENTRYPOINT ["/usr/local/bin/curloo"]