from rest_framework import serializers
from .models import Curso, Materia, Profesor, Salon

        
class MateriaSerializer(serializers.ModelSerializer):
    class Meta:
        model = Materia
        fields = '__all__'

class ProfesorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Profesor
        fields = '__all__'

class SalonSerializer(serializers.ModelSerializer):
    class Meta:
        model = Salon
        fields = '__all__'

class CursoSerializer(serializers.ModelSerializer):
    materia = MateriaSerializer()
    profesor = ProfesorSerializer()
    salon = SalonSerializer()

    class Meta:
        model = Curso
        fields = '__all__'
