# Stage 1: Build the application
FROM rust:1.81-slim-bullseye AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY entrypoint.sh ./entrypoint.sh
COPY .env .env

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary and other necessary files
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/.env .env

# Give execute permission to the entrypoint script
RUN chmod +x /app/entrypoint.sh

# Expose the port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]
