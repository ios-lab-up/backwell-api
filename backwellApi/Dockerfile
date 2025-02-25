# backwellApi/Dockerfile

# ----------------------
# Builder Stage
# ----------------------
FROM rust:1.81-slim-bullseye AS builder

# Set noninteractive mode and enable retries for apt-get
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -o Acquire::Retries=3 && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the Cargo manifest files for dependency caching.
COPY Cargo.toml Cargo.lock ./
# Ensure a target file exists so that Cargo recognizes a valid package.
RUN mkdir -p src && if [ ! -f src/main.rs ]; then echo "fn main() {}" > src/main.rs; fi

# Fetch dependencies (this layer is cached if Cargo.toml/lock remain unchanged)
RUN cargo fetch
# Build the binary target for caching (change --lib to --bin backwellApi)
RUN cargo build --release --bin backwellApi

# Now copy the full source code and rebuild the final binary.
COPY . .
RUN cargo build --release

# ----------------------
# Final Stage
# ----------------------
FROM debian:bullseye-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -o Acquire::Retries=3 && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage and verify it exists.
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/backwellApi
RUN if [ ! -f /usr/local/bin/backwellApi ]; then echo "Binary not found!" && exit 1; fi

# Expose the service port
EXPOSE 8082

# Run the binary (logs will be output to the container log)
CMD ["/usr/local/bin/backwellApi"]
