import django_filters
from .models import Curso

class CursoFilter(django_filters.FilterSet):
    materia__codigo = django_filters.CharFilter(field_name='materia__codigo', lookup_expr='exact')
    profesor__nombre = django_filters.CharFilter(field_name='profesor__nombre', lookup_expr='icontains')
    salon__nombre = django_filters.CharFilter(field_name='salon__nombre', lookup_expr='exact')
    ciclo = django_filters.NumberFilter(field_name='ciclo', lookup_expr='exact')
    sesion = django_filters.CharFilter(field_name='sesion', lookup_expr='exact')

    class Meta:
        model = Curso
        fields = ['materia__codigo', 'profesor__nombre', 'salon__nombre', 'ciclo', 'sesion']
