#!/bin/sh

set -e

# Esperar a que la base de datos esté lista
echo "Esperando a que PostgreSQL esté listo..."
until pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER"; do
  sleep 1
done
echo "PostgreSQL está listo."

# Ejecutar migraciones
echo "Ejecutando migraciones..."
/usr/local/bin/backwellApi migrate || { echo 'Error al ejecutar migraciones'; exit 1; }

# Iniciar la aplicación
echo "Iniciando la aplicación..."
exec /usr/local/bin/backwellApi || { echo 'Error al iniciar la aplicación'; exit 1; }
