FROM rust:bookworm AS builder

WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt install -y libpq-dev openssl ca-certificates
COPY --from=builder /build/target/release/carmine-api .
COPY --from=builder /build/carmine-api-airdrop/src/air-drop.json .

EXPOSE 8000

CMD ["./carmine-api"]
