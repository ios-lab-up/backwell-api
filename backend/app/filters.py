import django_filters
from .models import Curso

class CursoFilter(django_filters.FilterSet):
    materia__nombre = django_filters.CharFilter(field_name='materia__nombre', lookup_expr='icontains')
    profesor__nombre = django_filters.CharFilter(field_name='profesor__nombre', lookup_expr='icontains')
    ciclo = django_filters.NumberFilter(field_name='ciclo', lookup_expr='exact')
    sesion = django_filters.CharFilter(field_name='sesion', lookup_expr='exact')

    class Meta:
        model = Curso
        fields = ['materia__nombre', 'profesor__nombre', 'ciclo', 'sesion']
