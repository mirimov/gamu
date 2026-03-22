FROM rust:alpine

RUN apk add --no-cache musl-dev just git

RUN rustup component add rustfmt clippy

WORKDIR /volume