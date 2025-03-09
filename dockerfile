FROM rust:alpine AS builder

RUN apk add --no-cache openssl openssl-dev musl-dev pkgconf
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

WORKDIR /app

COPY Cargo.toml .
RUN cargo fetch
COPY src ./src

RUN cargo build --release


FROM alpine:latest

RUN apk add --no-cache openssl

COPY --from=builder /app/target/release/curloo /usr/local/bin/curloo

ENTRYPOINT ["/usr/local/bin/curloo"]