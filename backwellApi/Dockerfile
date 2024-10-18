# Etapa 1 y 2 utilizan la misma imagen base
FROM rust:1.81-slim-bullseye as builder

WORKDIR /app

# Instalar dependencias necesarias para compilar
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential

# Copiar los archivos de configuración de Rust
COPY Cargo.toml Cargo.lock ./

# Crear un archivo main.rs vacío para compilar las dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compilar las dependencias
RUN cargo build --release

# Eliminar el main.rs temporal
RUN rm -f src/main.rs

# Copiar el código fuente restante
COPY . .

# Compilar la aplicación
RUN cargo build --release

# Etapa 2: Crear la imagen final utilizando la misma imagen base
FROM rust:1.81-slim-bullseye

WORKDIR /app

# Instalar dependencias necesarias para ejecutar la aplicación
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado desde la etapa anterior
COPY --from=builder /app/target/release/backwellApi /usr/local/bin/

# Copiar el script entrypoint
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

# Dar permisos de ejecución al script
RUN chmod +x /app/entrypoint.sh

# Copiar las migraciones y otros archivos necesarios
COPY --from=builder /app/migrations ./migrations
COPY .env .env

# Exponer el puerto en el que tu aplicación escucha
EXPOSE 8080

# Establecer variables de entorno necesarias
ENV DATABASE_URL=postgres://user:password@db:5432/database_name
ENV POSTGRES_USER=user
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=database_name
ENV POSTGRES_HOST=db
ENV POSTGRES_PORT=5432

# Establecer el entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]

# Comando para ejecutar la aplicación (ya se incluye en entrypoint.sh)
# CMD ["backwellApi"]
