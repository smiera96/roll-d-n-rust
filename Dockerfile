# Build stage
#FROM rust:1.69-buster as builder
FROM rust:1.69 as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/roll-d-n-rust .

CMD ["./roll-d-n-rust"]
