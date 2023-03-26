FROM rust:buster as builder

WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

RUN apt-get update && apt install -y openssl ca-certificates
COPY --from=builder /build/target/release/carmine-api .

EXPOSE 8000

CMD ["./carmine-api"]
