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

            # Diccionario para rastrear cursos procesados por sus IDs
            courses_by_id = {}

            for _, row in df.iterrows():
                profesor_nombre = str(row['Profesor']).strip()
                if profesor_nombre == '':
                    # Saltar filas sin nombre de profesor
                    continue

                id_profesor = str(row['Id profesor']).strip() if 'Id profesor' in row else ''
                rol_profesor = str(row['Rol Profesor']).strip() if 'Rol Profesor' in row else ''

                # Obtener o crear el registro del profesor
                profesor, _ = Profesor.objects.get_or_create(
                    nombre=profesor_nombre,
                    defaults={'id_profesor': id_profesor}
                )

                materia_nombre = str(row['Clase']).strip()
                if not materia_nombre:
                    # Saltar filas sin nombre de materia
                    continue

                # Obtener o crear la materia
                materia, _ = Materia.objects.get_or_create(
                    nombre=materia_nombre,
                    defaults={
                        'no_de_catalogo': row['No de catálogo'],
                        'codigo': row['Materia']
                    }
                )

                # Procesar 'Clases Comb.'
                clases_comb_value = str(row['Clases Comb.']).strip()
                if clases_comb_value:
                    # Dividir los IDs combinados y eliminar espacios
                    clases_comb_list = [comb.strip() for comb in clases_comb_value.split(',') if comb.strip()]
                else:
                    clases_comb_list = []

                # Determinar el identificador principal del curso:
                # Usar 'Clases Comb.' si está disponible, de lo contrario usar 'No de clase'
                if clases_comb_list:
                    course_identifier = ','.join(sorted(clases_comb_list))
                else:
                    course_identifier = str(row['No de clase'])

                # Verificar si ya existe un curso con este identificador
                if course_identifier in courses_by_id:
                    # Actualizar el curso existente
                    curso = courses_by_id[course_identifier]
                else:
                    # Crear un nuevo curso
                    if 'Idioma en que se imparte la materia' in df.columns:
                        idioma_impartido = row['Idioma en que se imparte la materia']
                    else:
                        idioma_impartido = ''

                    # Nombre de la columna para 'Capacidad Inscripción Combinación'
                    # Dependiendo de la forma en que se formatea en el Excel
                    capacidad_comb_col = 'Capacidad Inscripción Combinación'
                    if capacidad_comb_col not in df.columns:
                        capacidad_comb_col = df.columns[df.columns.str.contains('Capacidad', case=False)].tolist()
                        capacidad_comb_col = capacidad_comb_col[0] if capacidad_comb_col else ''

                    # Crear los datos del curso
                    curso_data = {
                        'id_del_curso': str(row['Id del Curso']),
                        'ciclo': str(row['Ciclo']),
                        'sesion': str(row['Sesión']),
                        'seccion_clase': str(row['Sección Clase']),
                        'grupo_academico': str(row['Grupo académico']),
                        'organizacion_academica': str(row['Organización académica']),
                        'intercambio': str(row['Intercambio']),
                        'inter_plantel': str(row['Inter plantel']),
                        'oficialidad_materia': str(row['Oficialidad de la materia']),
                        'plan_academico': str(row['Plan Académico']),
                        'sede': str(row['Sede']),
                        'id_administrador_curso': str(row['Id Administrador de curso']),
                        'nombre_administrador_curso': str(row['Nombre de Administrador de curso']),
                        'materia': materia,
                        'mat_comb': str(row['Mat. Comb.']),
                        'clases_comb': clases_comb_value,
                        'capacidad_inscripcion_combinacion': row.get(capacidad_comb_col, None) if capacidad_comb_col else None,
                        'no_de_catalogo': str(row['No de catálogo']),
                        'clase': str(row['Clase']),
                        'no_de_clase': str(row['No de clase']),
                        'capacidad_inscripcion': row['Capacidad Inscripción'],
                        'total_inscripciones': row['Total  inscripciones'],
                        'total_inscripciones_materia_combinada': row['Total de inscripciones materia combinada'],
                        'fecha_inicial': row['Fecha inicial'] if pd.notnull(row['Fecha inicial']) else None,
                        'fecha_final': row['Fecha final'] if pd.notnull(row['Fecha final']) else None,
                        'bloque_optativo': str(row['Bloque optativo']),
                        'idioma_impartido': idioma_impartido,
                        'modalidad_clase': str(row['Modalidad de la clase']),
                    }

                    # Crear o actualizar el curso en la base de datos
                    curso, _ = Curso.objects.get_or_create(
                        no_de_clase=curso_data['no_de_clase'],
                        defaults=curso_data
                    )

                    # Agregar el curso al diccionario
                    courses_by_id[course_identifier] = curso

                # Asignar el profesor titular o adjunto al curso según sea necesario
                if rol_profesor.lower() == 'titular':
                    curso.profesor = profesor
                elif rol_profesor.lower() == 'adjunto':
                    curso.adjunto = profesor
                curso.save()

                # Procesar el horario de la fila actual
                days = ['Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado', 'Domingo']
                for day in days:
                    if row.get(day, '') == 'X':
                        dia = day

                        # Procesar horas
                        hora_inicio = str(row['Hora inicio']).strip()
                        hora_fin = str(row['Hora fin']).strip()
                        if hora_inicio and hora_fin:
                            try:
                                hora_inicio_dt = datetime.strptime(hora_inicio, '%I:%M %p').time()
                            except ValueError:
                                try:
                                    hora_inicio_dt = datetime.strptime(hora_inicio, '%H:%M').time()
                                except ValueError:
                                    self.stdout.write(self.style.ERROR(f"Error al parsear hora inicio en curso {curso.id_del_curso} - {dia}"))
                                    continue
                            try:
                                hora_fin_dt = datetime.strptime(hora_fin, '%I:%M %p').time()
                            except ValueError:
                                try:
                                    hora_fin_dt = datetime.strptime(hora_fin, '%H:%M').time()
                                except ValueError:
                                    self.stdout.write(self.style.ERROR(f"Error al parsear hora fin en curso {curso.id_del_curso} - {dia}"))
                                    continue
                        else:
                            # Si faltan horas, saltar la fila
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
                            defaults={'capacidad': row.get('Capacidad del salón', None)}
                        )

                        # Crear o actualizar el horario
                        Schedule.objects.get_or_create(
                            curso=curso,
                            dia=dia,
                            hora_inicio=hora_inicio_dt,
                            hora_fin=hora_fin_dt,
                            salon=salon
                        )

            self.stdout.write(self.style.SUCCESS('Todos los datos han sido importados exitosamente'))

        except Exception as e:
            logger.error(f"Error al importar datos: {e}")
            self.stdout.write(self.style.ERROR(f"Error al importar datos: {e}"))
