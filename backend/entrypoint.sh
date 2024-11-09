#!/bin/sh
set -e

echo "Esperando a que la base de datos Postgres se inicie..."

while ! nc -z $DB_HOST $DB_PORT; do
  sleep 0.1
done

echo "Base de datos Postgres iniciada"
echo "Checking for changes that require migrations..."
python manage.py makemigrations --dry-run | grep 'No changes detected' || {
  echo "Creating migrations..."
  python manage.py makemigrations app
  echo "<==================================>"
}
# Ejecutar migraciones
python manage.py migrate

# Ejecutar el comando de importación del Excel
python manage.py import_excel

# Iniciar el servidor
python manage.py runserver 0.0.0.0:8000
