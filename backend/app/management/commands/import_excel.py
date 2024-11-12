# backend/app/management/commands/import_excel.py

import pandas as pd
from django.core.management.base import BaseCommand
from app.models import Materia, Profesor, Salon, Curso, Schedule
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
            try:
                # Leer el Excel, ajustando el número de fila del encabezado según sea necesario
                df = pd.read_excel(excel_file_path, header=9)
                df.columns = df.columns.str.strip()  # Eliminar espacios en los nombres de columnas
            except Exception as e:
                self.stdout.write(self.style.ERROR(f"Error al leer el archivo Excel: {e}"))
                return

            # Preprocesar el DataFrame
            for column in df.select_dtypes(include=['float64']).columns:
                df[column] = df[column].astype(object)
            df.fillna('', inplace=True)

            # Agrupar por 'No de clase' y 'Profesor'
            grouped = df.groupby(['No de clase', 'Profesor'])
            for (no_de_clase, profesor_nombre), group in grouped:
                profesor_nombre = profesor_nombre.strip()
                id_profesor = str(group['Id profesor'].iloc[0]).strip()
                rol_profesor = str(group['Rol Profesor'].iloc[0]).strip()

                profesor, _ = Profesor.objects.get_or_create(
                    nombre=profesor_nombre,
                    defaults={
                        'id_profesor': id_profesor,
                    }
                )

                materia_nombre = group['Clase'].iloc[0].strip()
                materia, _ = Materia.objects.get_or_create(
                    nombre=materia_nombre,
                    defaults={
                        'no_de_catalogo': group['No de catálogo'].iloc[0],
                        'codigo': group['Materia'].iloc[0]
                    }
                )

                # Verificar si la columna 'Idioma en que se imparte la materia' existe
                if 'Idioma en que se imparte la materia' in group.columns:
                    idioma_impartido = group['Idioma en que se imparte la materia'].iloc[0]
                else:
                    idioma_impartido = ''
                
                nombre_columna = "Capacidad\nInscripción\nCombinación"

                curso_data = {
                    'id_del_curso': group['Id del Curso'].iloc[0],
                    'ciclo': group['Ciclo'].iloc[0],
                    'sesion': group['Sesión'].iloc[0],
                    'seccion_clase': group['Sección Clase'].iloc[0],
                    'grupo_academico': group['Grupo académico'].iloc[0],
                    'organizacion_academica': group['Organización académica'].iloc[0],
                    'intercambio': group['Intercambio'].iloc[0],
                    'inter_plantel': group['Inter plantel'].iloc[0],
                    'oficialidad_materia': group['Oficialidad de la materia'].iloc[0],
                    'plan_academico': group['Plan Académico'].iloc[0],
                    'sede': group['Sede'].iloc[0],
                    'id_administrador_curso': group['Id Administrador de curso'].iloc[0],
                    'nombre_administrador_curso': group['Nombre de Administrador de curso'].iloc[0],
                    'rol_profesor': rol_profesor,
                    'materia': materia,
                    'mat_comb': group['Mat. Comb.'].iloc[0],
                    'clases_comb': group['Clases Comb.'].iloc[0],
                    'capacidad_inscripcion_combinacion': group[nombre_columna].iloc[0],
                    'no_de_catalogo': group['No de catálogo'].iloc[0],
                    'clase': group['Clase'].iloc[0],
                    'no_de_clase': no_de_clase,
                    'capacidad_inscripcion': group['Capacidad Inscripción'].iloc[0],
                    'total_inscripciones': group['Total  inscripciones'].iloc[0],
                    'total_inscripciones_materia_combinada': group['Total de inscripciones materia combinada'].iloc[0],
                    'fecha_inicial': group['Fecha inicial'].iloc[0],
                    'fecha_final': group['Fecha final'].iloc[0],
                    'bloque_optativo': group['Bloque optativo'].iloc[0],
                    'idioma_impartido': idioma_impartido,
                    'modalidad_clase': group['Modalidad de la clase'].iloc[0],
                    'profesor': profesor,
                }

                curso, created = Curso.objects.get_or_create(
                    no_de_clase=no_de_clase,
                    profesor=profesor,
                    defaults=curso_data
                )

                for index, row in group.iterrows():
                    days = ['Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado', 'Domingo']
                    for day in days:
                        if row[day] == 'X':
                            dia = day

                            # Procesar horas
                            try:
                                hora_inicio = datetime.strptime(str(row['Hora inicio']), '%I:%M %p').time()
                                hora_fin = datetime.strptime(str(row['Hora fin']), '%I:%M %p').time()
                            except ValueError:
                                try:
                                    hora_inicio = datetime.strptime(str(row['Hora inicio']), '%H:%M').time()
                                    hora_fin = datetime.strptime(str(row['Hora fin']), '%H:%M').time()
                                except ValueError:
                                    self.stdout.write(self.style.ERROR(f"Error al parsear hora en curso {curso.id_del_curso}"))
                                    continue

                            # Procesar salón
                            salon_nombre = str(row['Salón']).strip()
                            modalidad_clase = str(row['Modalidad de la clase']).strip().upper()

                            if modalidad_clase in ['ENLINEA', 'EN LÍNEA']:
                                salon_nombre = 'En Línea'
                            elif not salon_nombre:
                                salon_nombre = 'Sin Asignar'

                            salon, _ = Salon.objects.get_or_create(
                                nombre=salon_nombre,
                                defaults={'capacidad': row['Capacidad del salón']}
                            )

                            # Crear horario
                            schedule, _ = Schedule.objects.get_or_create(
                                curso=curso,
                                dia=dia,
                                hora_inicio=hora_inicio,
                                hora_fin=hora_fin,
                                salon=salon,
                                profesor=profesor
                            )

                # self.stdout.write(self.style.SUCCESS(f'Curso {curso.id_del_curso} importado exitosamente'))
                # self.stdout.write(self.style.SUCCESS(f'Horarios del curso {schedule} importados exitosamente'))

            self.stdout.write(self.style.SUCCESS('Todos los datos han sido importados exitosamente'))

        except Exception as e:
            logger.error(f"Error al importar datos: {e}")
            self.stdout.write(self.style.ERROR(f"Error al importar datos: {e}"))
