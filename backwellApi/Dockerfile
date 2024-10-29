# Use Rust as the builder image
FROM rust:1.81-slim-bullseye AS builder

# Install necessary packages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the manifest and lock files and create a dummy src folder for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Copy the rest of the source files and build the actual app
COPY . .
RUN cargo build --release

# Use a smaller image to run the application
FROM debian:bullseye-slim

# Install the necessary libraries for the compiled application
RUN apt-get update && apt-get install -y ca-certificates\
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/backwellApi

# Expose the application port
EXPOSE 8082

# Run the application
CMD ["/usr/local/bin/backwellApi"]
