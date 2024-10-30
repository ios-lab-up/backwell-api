import pandas as pd
from django.core.management.base import BaseCommand
from app.models import Materia, Profesor, Salon, Curso
from django.utils import timezone
from datetime import datetime
import os
from django.conf import settings
import logging
logger = logging.getLogger('app')

class Command(BaseCommand):
    help = 'Importa datos desde un archivo Excel a la base de datos'

    def handle(self, *args, **kwargs):
        try:
            excel_file_path = os.path.join(settings.BASE_DIR, 'Schedule.xlsx')
            self.stdout.write(f"Buscando el archivo Excel en: {excel_file_path}")
            try:
                df = pd.read_excel(excel_file_path)
            except Exception as e:
                self.stdout.write(self.style.ERROR(f"Error al leer el archivo Excel: {e}"))
                return
            # Procesa cada fila del DataFrame
            for index, row in df.iterrows():
                # Crear o obtener el profesor
                profesor, _ = Profesor.objects.get_or_create(nombre=row['Profesor'])

                # Crear o obtener el salón
                salon, _ = Salon.objects.get_or_create(nombre=row['Salón'], defaults={'capacidad': row['Capacidad del salón']})

                # Crear o obtener la materia
                materia_obj, _ = Materia.objects.get_or_create(
                    codigo=row['Materia'],
                    defaults={
                        'nombre': row['Clase'],
                        'no_de_catalogo': row['No de catálogo']
                    }
                )

                # Convertir campos de fecha y hora
                fecha_inicial = datetime.strptime(str(row['Fecha inicial']), '%Y-%m-%d')
                fecha_final = datetime.strptime(str(row['Fecha final']), '%Y-%m-%d')
                hora_inicio = datetime.strptime(str(row['Hora inicio']), '%I:%M %p').time()
                hora_fin = datetime.strptime(str(row['Hora fin']), '%I:%M %p').time()

                # Convertir campos de días de la semana
                dias = {
                    'lunes': row['Lunes'] == 'X',
                    'martes': row['Martes'] == 'X',
                    'miercoles': row['Miércoles'] == 'X',
                    'jueves': row['Jueves'] == 'X',
                    'viernes': row['Viernes'] == 'X',
                    'sabado': row['Sábado'] == 'X',
                    'domingo': False  # Siempre vacío
                }

                # Crear el curso
                Curso.objects.create(
                    id_del_curso=row['Id del Curso'],
                    ciclo=row['Ciclo'],
                    sesion=row['Sesión'],
                    materia=materia_obj,
                    mat_comb=row['Mat. Comb.'],
                    clases_comb=row['Clases Comb.'],
                    capacidad_inscripcion_combinacion=row['Capacidad\nInscripción\nCombinación'],
                    no_de_catalogo=row['No de catálogo'],
                    clase=row['Clase'],
                    no_de_clase=row['No de clase'],
                    capacidad_inscripcion=row['Capacidad Inscripción'],
                    total_inscripciones=row['Total  inscripciones'],
                    total_inscripciones_materia_combinada=row['Total de inscripciones materia combinada'],
                    fecha_inicial=fecha_inicial,
                    fecha_final=fecha_final,
                    salon=salon,
                    capacidad_del_salon=salon.capacidad,
                    hora_inicio=hora_inicio,
                    hora_fin=hora_fin,
                    profesor=profesor,
                    bloque_optativo=row['Bloque optativo'],
                    modalidad_clase=row['Modalidad de la clase'],
                    **dias
                )

            self.stdout.write(self.style.SUCCESS('Datos importados exitosamente'))
        except Exception as e:
            logger.error(f"Error al importar datos: {e}")
            self.stdout.write(self.style.ERROR(f"Error al importar datos: {e}"))
