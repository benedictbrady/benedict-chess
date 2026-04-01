FROM rust:1.86-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release -p benedict-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/benedict-api /usr/local/bin/benedict-api

ENV PORT=10000
EXPOSE 10000

CMD ["benedict-api"]
