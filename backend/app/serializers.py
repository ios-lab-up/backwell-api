# backend/app/serializers.py

from rest_framework import serializers
from .models import Curso, Materia, Profesor, Salon, Schedule

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

class ScheduleSerializer(serializers.ModelSerializer):
    salon = SalonSerializer()

    class Meta:
        model = Schedule
        fields = '__all__'

class CursoSerializer(serializers.ModelSerializer):
    materia = MateriaSerializer()
    profesor = ProfesorSerializer()
    schedules = ScheduleSerializer(many=True)
    horario = serializers.SerializerMethodField()

    class Meta:
        model = Curso
        fields = '__all__'

    def get_horario(self, obj):
        return obj.get_horario()
