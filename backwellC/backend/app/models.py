from django.db import models

class Materia(models.Model):
    codigo = models.CharField(max_length=50)  # Campo 'Materia' en el Excel
    nombre = models.CharField(max_length=255)  # Campo 'Clase' en el Excel
    no_de_catalogo = models.CharField(max_length=50)  # Campo 'No de catálogo'

    def __str__(self):
        return f"{self.nombre} ({self.codigo})"

class Salon(models.Model):
    nombre = models.CharField(max_length=50)
    capacidad = models.IntegerField(null=True, blank=True)

    def __str__(self):
        return self.nombre

class Profesor(models.Model):
    nombre = models.CharField(max_length=255)

    def __str__(self):
        return self.nombre

class Curso(models.Model):
    id_del_curso = models.IntegerField(verbose_name='Id del Curso')
    ciclo = models.IntegerField(verbose_name='Ciclo')
    sesion = models.CharField(max_length=50, verbose_name='Sesión')
    materia = models.ForeignKey(Materia, on_delete=models.CASCADE, verbose_name='Materia')
    mat_comb = models.IntegerField(verbose_name='Mat. Comb.')
    clases_comb = models.CharField(max_length=255, verbose_name='Clases Comb.')
    capacidad_inscripcion_combinacion = models.IntegerField(verbose_name='Capacidad Inscripción Combinación')
    no_de_catalogo = models.CharField(max_length=50, verbose_name='No de catálogo')
    clase = models.CharField(max_length=100, verbose_name='Clase')
    no_de_clase = models.IntegerField(verbose_name='No de clase')
    capacidad_inscripcion = models.IntegerField(verbose_name='Capacidad Inscripción')
    total_inscripciones = models.IntegerField(verbose_name='Total inscripciones')
    total_inscripciones_materia_combinada = models.IntegerField(verbose_name='Total de inscripciones materia combinada')
    fecha_inicial = models.DateField(verbose_name='Fecha inicial')
    fecha_final = models.DateField(verbose_name='Fecha final')
    salon = models.ForeignKey(Salon, on_delete=models.SET_NULL, null=True, blank=True, verbose_name='Salón')
    capacidad_del_salon = models.IntegerField(verbose_name='Capacidad del salón')
    hora_inicio = models.TimeField(verbose_name='Hora inicio')
    hora_fin = models.TimeField(verbose_name='Hora fin')
    profesor = models.ForeignKey(Profesor, on_delete=models.SET_NULL, null=True, blank=True, verbose_name='Profesor')
    lunes = models.BooleanField(default=False, verbose_name='Lunes')
    martes = models.BooleanField(default=False, verbose_name='Martes')
    miercoles = models.BooleanField(default=False, verbose_name='Miércoles')
    jueves = models.BooleanField(default=False, verbose_name='Jueves')
    viernes = models.BooleanField(default=False, verbose_name='Viernes')
    sabado = models.BooleanField(default=False, verbose_name='Sábado')
    domingo = models.BooleanField(default=False, verbose_name='Domingo')
    bloque_optativo = models.CharField(max_length=50, verbose_name='Bloque optativo')
    idioma_impartido = models.CharField(max_length=50, blank=True, null=True, verbose_name='Idioma en que se imparte la materia')
    modalidad_clase = models.CharField(max_length=50, blank=True, null=True, verbose_name='Modalidad de la clase')

    def __str__(self):
        return f"Curso {self.id_del_curso} - {self.materia.nombre}"
