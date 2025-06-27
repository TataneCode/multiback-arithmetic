FROM rust:1.86 as builder

RUN apt-get update && apt-get install -y \
    musl-tools \
    curl build-essential pkg-config libssl-dev ca-certificates \
    && update-ca-certificates

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crypto-api .

EXPOSE 8080
CMD ["./crypto-api"]
