# backwellApi/Dockerfile

# Etapa de construcción
FROM rust:1.81-slim-bullseye AS builder

# Instalar paquetes necesarios
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar archivos de dependencia y crear un archivo main.rs temporal
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compilar dependencias
RUN cargo build --release

# Copiar el código fuente y compilar la aplicación real
COPY . .
RUN cargo build --release

# Etapa final
FROM debian:bullseye-slim

# Instalar librerías necesarias
RUN apt-get update && apt-get install -y ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/backwellApi

# Exponer el puerto de la aplicación
EXPOSE 8082

# Ejecutar la aplicación
ENTRYPOINT ["/usr/local/bin/backwellApi"]
