FROM rust:1.71-alpine

RUN apk add --update \
    alpine-sdk \
    ffmpeg \
    youtube-dl \
    pkgconfig \
    cmake \
    openssl-dev \
    musl-dev \
    openssl

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/melvin_rs"]