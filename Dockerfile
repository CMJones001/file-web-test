# syntax: docker/dockerfile:1

FROM rust:1.68-buster

WORKDIR /app

COPY . .

RUN cargo build --release