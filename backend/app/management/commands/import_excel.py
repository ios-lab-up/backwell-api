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
            # Check if the database is empty, if it is populate it with the data from the Excel file if not skip the process and dont import the data
            if Curso.objects.exists():
                self.stdout.write(self.style.WARNING('La base de datos ya contiene datos, no se importará nada'))
                return

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

            # Determinar el identificador principal del curso para cada fila
            def get_course_identifier(row):
                clases_comb_value = str(row['Clases Comb.']).strip()
                if clases_comb_value:
                    # Dividir los IDs combinados y eliminar espacios
                    clases_comb_list = [comb.strip() for comb in clases_comb_value.split(',') if comb.strip()]
                    return ','.join(sorted(clases_comb_list))
                else:
                    return str(row['No de clase'])

            df['course_identifier'] = df.apply(get_course_identifier, axis=1)

            # Agrupar filas por el identificador de curso
            grouped_courses = df.groupby('course_identifier')

            for course_identifier, group in grouped_courses:
                # Omite grupos sin información relevante
                if course_identifier == '' or group.empty:
                    continue

                # Combina datos de curso de todas las filas del grupo
                course_data = {}
                schedules_data = []
                profesor_titular = None
                profesor_adjunto = None

                for _, row in group.iterrows():
                    profesor_nombre = str(row['Profesor']).strip()
                    if profesor_nombre == '':
                        # Saltar filas sin nombre de profesor (posibles placeholders)
                        continue

                    id_profesor = str(row['Id profesor']).strip() if 'Id profesor' in row else ''
                    rol_profesor = str(row['Rol Profesor']).strip() if 'Rol Profesor' in row else ''

                    # Obtener o crear el registro del profesor
                    profesor, _ = Profesor.objects.get_or_create(
                        nombre=profesor_nombre,
                        defaults={'id_profesor': id_profesor}
                    )

                    # Asignar el profesor titular o adjunto al curso según sea necesario
                    if rol_profesor.lower() == 'titular':
                        profesor_titular = profesor
                    elif rol_profesor.lower() == 'adjunto':
                        profesor_adjunto = profesor

                    materia_nombre = str(row['Clase']).strip()
                    if not materia_nombre:
                        # Saltar filas sin nombre de materia (posibles placeholders)
                        continue

                    # Obtener o crear la materia
                    materia, _ = Materia.objects.get_or_create(
                        nombre=materia_nombre,
                        defaults={
                            'no_de_catalogo': row['No de catálogo'],
                            'codigo': row['Materia']
                        }
                    )

                    # Nombre de la columna para 'Capacidad Inscripción Combinación'
                    # Dependiendo de la forma en que se formatea en el Excel
                    capacidad_comb_col = 'Capacidad Inscripción Combinación'
                    if capacidad_comb_col not in df.columns:
                        capacidad_comb_col = df.columns[df.columns.str.contains('Capacidad', case=False)].tolist()
                        capacidad_comb_col = capacidad_comb_col[0] if capacidad_comb_col else ''

                    # Actualizar o establecer datos del curso si no existen o son valores predeterminados
                    course_data['id_del_curso'] = course_data.get('id_del_curso', str(row['Id del Curso']))
                    course_data['ciclo'] = course_data.get('ciclo', str(row['Ciclo']))
                    course_data['sesion'] = course_data.get('sesion', str(row['Sesión']))
                    course_data['seccion_clase'] = course_data.get('seccion_clase', str(row['Sección Clase']))
                    course_data['grupo_academico'] = course_data.get('grupo_academico', str(row['Grupo académico']))
                    course_data['organizacion_academica'] = course_data.get('organizacion_academica', str(row['Organización académica']))
                    course_data['intercambio'] = course_data.get('intercambio', str(row['Intercambio']))
                    course_data['inter_plantel'] = course_data.get('inter_plantel', str(row['Inter plantel']))
                    course_data['oficialidad_materia'] = course_data.get('oficialidad_materia', str(row['Oficialidad de la materia']))
                    course_data['plan_academico'] = course_data.get('plan_academico', str(row['Plan Académico']))
                    course_data['sede'] = course_data.get('sede', str(row['Sede']))
                    course_data['id_administrador_curso'] = course_data.get('id_administrador_curso', str(row['Id Administrador de curso']))
                    course_data['nombre_administrador_curso'] = course_data.get('nombre_administrador_curso', str(row['Nombre de Administrador de curso']))
                    course_data['materia'] = course_data.get('materia', materia)
                    course_data['mat_comb'] = course_data.get('mat_comb', str(row['Mat. Comb.']))
                    course_data['clases_comb'] = course_data.get('clases_comb', str(row['Clases Comb.']))
                    if capacidad_comb_col:
                        course_data['capacidad_inscripcion_combinacion'] = course_data.get('capacidad_inscripcion_combinacion', row.get(capacidad_comb_col, None))
                    course_data['no_de_catalogo'] = course_data.get('no_de_catalogo', str(row['No de catálogo']))
                    course_data['clase'] = course_data.get('clase', str(row['Clase']))
                    course_data['no_de_clase'] = course_identifier  # Use course_identifier as no_de_clase
                    course_data['capacidad_inscripcion'] = course_data.get('capacidad_inscripcion', row['Capacidad Inscripción'])
                    course_data['total_inscripciones'] = course_data.get('total_inscripciones', row['Total  inscripciones'])
                    course_data['total_inscripciones_materia_combinada'] = course_data.get('total_inscripciones_materia_combinada', row['Total de inscripciones materia combinada'])
                    course_data['fecha_inicial'] = course_data.get('fecha_inicial', row['Fecha inicial'] if pd.notnull(row['Fecha inicial']) else None)
                    course_data['fecha_final'] = course_data.get('fecha_final', row['Fecha final'] if pd.notnull(row['Fecha final']) else None)
                    course_data['bloque_optativo'] = course_data.get('bloque_optativo', str(row['Bloque optativo']))
                    # Verificar si la columna 'Idioma en que se imparte la materia' existe
                    if 'Idioma en que se imparte la materia' in df.columns:
                        course_data['idioma_impartido'] = course_data.get('idioma_impartido', row['Idioma en que se imparte la materia'])
                    else:
                        course_data['idioma_impartido'] = course_data.get('idioma_impartido', '')
                    course_data['modalidad_clase'] = course_data.get('modalidad_clase', str(row['Modalidad de la clase']))

                    # Procesar horarios solo si la fila contiene información de horario
                    days = ['Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado', 'Domingo']
                    day_found = False
                    for day in days:
                        if row.get(day, '') == 'X':
                            day_found = True
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
                                        self.stdout.write(self.style.ERROR(f"Error al parsear hora inicio en curso {course_identifier} - {dia}"))
                                        continue
                                try:
                                    hora_fin_dt = datetime.strptime(hora_fin, '%I:%M %p').time()
                                except ValueError:
                                    try:
                                        hora_fin_dt = datetime.strptime(hora_fin, '%H:%M').time()
                                    except ValueError:
                                        self.stdout.write(self.style.ERROR(f"Error al parsear hora fin en curso {course_identifier} - {dia}"))
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

                            schedules_data.append({
                                'dia': dia,
                                'hora_inicio': hora_inicio_dt,
                                'hora_fin': hora_fin_dt,
                                'salon': salon
                            })

                    # Si la fila no marcó ningún día con 'X' y no se encontró horario, es posible que sea un placeholder
                    # No procesar más datos de esa fila
                    if not day_found:
                        continue

                # Ahora que hemos recopilado todos los datos del curso y los horarios, crear o actualizar el curso
                if not course_data:
                    # Si no hay datos del curso, omite este grupo
                    continue

                # Crear o actualizar el curso en la base de datos
                curso, created = Curso.objects.update_or_create(
                    no_de_clase=course_data['no_de_clase'],
                    defaults=course_data
                )

                # Asignar profesores titular y adjunto al curso
                if profesor_titular:
                    curso.profesor = profesor_titular
                if profesor_adjunto:
                    curso.adjunto = profesor_adjunto
                curso.save()

                # Crear o actualizar los horarios del curso
                for schedule_info in schedules_data:
                    Schedule.objects.get_or_create(
                        curso=curso,
                        dia=schedule_info['dia'],
                        hora_inicio=schedule_info['hora_inicio'],
                        hora_fin=schedule_info['hora_fin'],
                        salon=schedule_info['salon']
                    )

            self.stdout.write(self.style.SUCCESS('Todos los datos han sido importados exitosamente'))

        except Exception as e:
            logger.error(f"Error al importar datos: {e}")
            self.stdout.write(self.style.ERROR(f"Error al importar datos: {e}"))
