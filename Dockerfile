FROM rust:1.87-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build the application with optimizations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/commitkit /app/commitkit

ENTRYPOINT ["/app/commitkit"]
CMD ["--help"]