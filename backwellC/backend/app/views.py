from rest_framework import viewsets
from .models import Curso, Materia, Profesor, Salon
from .serializers import CursoSerializer, MateriaSerializer, ProfesorSerializer, SalonSerializer
from django_filters.rest_framework import DjangoFilterBackend
from .filters import CursoFilter

class CursoViewSet(viewsets.ModelViewSet):
    queryset = Curso.objects.all()
    serializer_class = CursoSerializer
    filter_backends = [DjangoFilterBackend]
    filterset_class = CursoFilter

class MateriaViewSet(viewsets.ModelViewSet):
    queryset = Materia.objects.all()
    serializer_class = MateriaSerializer

class ProfesorViewSet(viewsets.ModelViewSet):
    queryset = Profesor.objects.all()
    serializer_class = ProfesorSerializer

class SalonViewSet(viewsets.ModelViewSet):
    queryset = Salon.objects.all()
    serializer_class = SalonSerializer
