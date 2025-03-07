FROM rust:alpine as builder
WORKDIR /app

RUN cargo init --bin
COPY Cargo.toml .
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/curloo /usr/local/bin/
ENTRYPOINT ["curloo"]