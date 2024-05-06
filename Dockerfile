# Build stage
FROM rust:1.69 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/roll-d-n-rust .

CMD ["./roll-d-n-rust"]