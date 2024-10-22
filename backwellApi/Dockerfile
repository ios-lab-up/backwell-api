# =========================
# Stage 1: Builder
# =========================
FROM rust:1.81-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential \
    pkg-config

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Create a new directory for the application
WORKDIR /app

# Copy Cargo.toml and Cargo.lock to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (this step is cached unless Cargo.toml or Cargo.lock changes)
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code and other necessary files
COPY . .

# Build the actual application (this step will re-run when source code changes)
RUN cargo build --release

# =========================
# Stage 2: Runtime
# =========================
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/backwellApi

# Copy the diesel binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy other necessary files
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh
COPY --from=builder /app/migrations ./migrations

# Ensure the entrypoint script has execute permissions
RUN chmod +x /app/entrypoint.sh

# Expose the application port (adjust if your app uses a different port)
EXPOSE 8080

# Define the entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]
