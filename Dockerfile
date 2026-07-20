FROM rust:1.85-slim AS builder

WORKDIR /silf
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /silf
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /silf/target/release/main /silf/silf
COPY .env /silf/.env

EXPOSE 8000

CMD ["./silf"]
