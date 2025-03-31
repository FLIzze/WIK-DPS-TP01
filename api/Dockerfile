FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY ./main.rs ./main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /app/api

RUN adduser -D appuser
USER appuser

CMD ["/app/api"]
