# backend/app/models.py

from django.db import models

class Materia(models.Model):
    nombre = models.CharField(max_length=255, unique=True)
    no_de_catalogo = models.CharField(max_length=50, blank=True, null=True)
    codigo = models.CharField(max_length=50, blank=True, null=True)

    def __str__(self):
        return self.nombre

class Profesor(models.Model):
    nombre = models.CharField(max_length=255, unique=True)
    id_profesor = models.CharField(max_length=50, blank=True, null=True)

    def __str__(self):
        return self.nombre

class Salon(models.Model):
    nombre = models.CharField(max_length=50, unique=True)
    capacidad = models.IntegerField(null=True, blank=True)

    def __str__(self):
        return self.nombre

class Curso(models.Model):
    id_del_curso = models.CharField(max_length=50, verbose_name='Id del Curso')
    ciclo = models.CharField(max_length=50, verbose_name='Ciclo', blank=True, null=True)
    sesion = models.CharField(max_length=50, verbose_name='Sesión', blank=True, null=True)
    seccion_clase = models.CharField(max_length=100, verbose_name='Sección Clase', blank=True, null=True)
    grupo_academico = models.CharField(max_length=100, verbose_name='Grupo académico', blank=True, null=True)
    organizacion_academica = models.CharField(max_length=100, verbose_name='Organización académica', blank=True, null=True)
    intercambio = models.CharField(max_length=50, blank=True, null=True)
    inter_plantel = models.CharField(max_length=50, blank=True, null=True)
    oficialidad_materia = models.CharField(max_length=100, blank=True, null=True)
    plan_academico = models.CharField(max_length=100, blank=True, null=True)
    sede = models.CharField(max_length=100, blank=True, null=True)
    id_administrador_curso = models.CharField(max_length=50, blank=True, null=True)
    nombre_administrador_curso = models.CharField(max_length=255, blank=True, null=True)
    rol_profesor = models.CharField(max_length=100, blank=True, null=True, verbose_name='Rol Profesor')
    materia = models.ForeignKey(Materia, on_delete=models.CASCADE, verbose_name='Materia')
    mat_comb = models.CharField(max_length=255, verbose_name='Mat. Comb.', blank=True, null=True)
    clases_comb = models.CharField(max_length=255, verbose_name='Clases Comb.', blank=True, null=True)
    capacidad_inscripcion_combinacion = models.IntegerField(verbose_name='Capacidad Inscripción Combinación', blank=True, null=True)
    no_de_catalogo = models.CharField(max_length=50, verbose_name='No de catálogo', blank=True, null=True)
    clase = models.CharField(max_length=100, verbose_name='Clase', blank=True, null=True)
    no_de_clase = models.CharField(max_length=50, verbose_name='No de clase')
    capacidad_inscripcion = models.IntegerField(verbose_name='Capacidad Inscripción', blank=True, null=True)
    total_inscripciones = models.IntegerField(verbose_name='Total inscripciones', blank=True, null=True)
    total_inscripciones_materia_combinada = models.IntegerField(verbose_name='Total de inscripciones materia combinada', blank=True, null=True)
    fecha_inicial = models.DateField(verbose_name='Fecha inicial', blank=True, null=True)
    fecha_final = models.DateField(verbose_name='Fecha final', blank=True, null=True)
    bloque_optativo = models.CharField(max_length=50, verbose_name='Bloque optativo', blank=True, null=True)
    idioma_impartido = models.CharField(max_length=50, blank=True, null=True, verbose_name='Idioma en que se imparte la materia')
    modalidad_clase = models.CharField(max_length=50, blank=True, null=True, verbose_name='Modalidad de la clase')
    profesor = models.ForeignKey(Profesor, on_delete=models.SET_NULL, null=True, blank=True, verbose_name='Profesor')

    def __str__(self):
        return f"Curso {self.id_del_curso} - {self.materia.nombre} - Clase {self.no_de_clase}"

    def get_horario(self):
        horarios = self.schedules.all()
        horario_str = ' '.join([
            f"{schedule.dia} {schedule.hora_inicio.strftime('%H:%M')} - {schedule.hora_fin.strftime('%H:%M')}"
            for schedule in horarios
        ])
        return horario_str

class Schedule(models.Model):
    curso = models.ForeignKey(Curso, on_delete=models.CASCADE, related_name='schedules')
    dia = models.CharField(max_length=10, choices=[
        ('Lunes', 'Lunes'),
        ('Martes', 'Martes'),
        ('Miércoles', 'Miércoles'),
        ('Jueves', 'Jueves'),
        ('Viernes', 'Viernes'),
        ('Sábado', 'Sábado'),
        ('Domingo', 'Domingo'),
    ])
    hora_inicio = models.TimeField(verbose_name='Hora inicio')
    hora_fin = models.TimeField(verbose_name='Hora fin')
    salon = models.ForeignKey(Salon, on_delete=models.SET_NULL, null=True, blank=True, verbose_name='Salón')
    profesor = models.ForeignKey(Profesor, on_delete=models.SET_NULL, null=True, blank=True, verbose_name='Profesor')

    def __str__(self):
        return f"{self.dia} {self.hora_inicio.strftime('%H:%M')} - {self.hora_fin.strftime('%H:%M')}"
