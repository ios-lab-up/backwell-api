from django.contrib import admin
from .models import Curso, Schedule

@admin.register(Curso)
class CursoAdmin(admin.ModelAdmin):
    list_display = ('id_del_curso', 'materia', 'clase', 'profesor')
    search_fields = ('id_del_curso', 'materia__nombre', 'clase', 'profesor__nombre')

@admin.register(Schedule)
class ScheduleAdmin(admin.ModelAdmin):
    list_display = ('curso', 'dia', 'hora_inicio', 'hora_fin', 'salon', 'profesor')
    search_fields = ('curso__id_del_curso', 'dia', 'salon__nombre', 'profesor__nombre')
