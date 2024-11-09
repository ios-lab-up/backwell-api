from django.urls import path, include
from rest_framework import routers
from .views import CursoViewSet, MateriaViewSet, ProfesorViewSet, SalonViewSet

router = routers.DefaultRouter()
router.register(r'cursos', CursoViewSet)
router.register(r'materias', MateriaViewSet)
router.register(r'profesores', ProfesorViewSet)
router.register(r'salones', SalonViewSet)

urlpatterns = [
    path('', include(router.urls)),
]
