from django.contrib import admin
from .models import Curso

@admin.register(Curso)
class CursoAdmin(admin.ModelAdmin):
    list_display = ('id_del_curso', 'materia', 'clase', 'profesor', 'salon')
    search_fields = ('id_del_curso', 'materia', 'clase', 'profesor', 'salon')
