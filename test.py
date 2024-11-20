import pandas as pd

# new_Schedules data
new_Schedules = {
    "schedule_number": 1,
    "courses": [
        {
            "materia": "Álgebra",
            "profesor": "Guillén Santiago,Alejandro",
            "schedules": [
                {
                    "dia": "Lunes",
                    "hora_inicio": "13:00:00",
                    "hora_fin": "14:30:00",
                    "salon": "R 32"
                },
                {
                    "dia": "Miércoles",
                    "hora_inicio": "13:00:00",
                    "hora_fin": "14:30:00",
                    "salon": "NR 08"
                },
                {
                    "dia": "Viernes",
                    "hora_inicio": "13:00:00",
                    "hora_fin": "14:30:00",
                    "salon": "NR 08"
                }
            ]
        },
        {
            "materia": "Química",
            "profesor": "Real Lira,Lourdes María del Socorro",
            "schedules": [
                {
                    "dia": "Martes",
                    "hora_inicio": "10:00:00",
                    "hora_fin": "11:30:00",
                    "salon": "R 06"
                },
                {
                    "dia": "Jueves",
                    "hora_inicio": "10:00:00",
                    "hora_fin": "11:30:00",
                    "salon": "R 47"
                }
            ]
        },
        {
            "materia": "Mecánica",
            "profesor": "Reyes Razo,Ricardo",
            "schedules": [
                {
                    "dia": "Lunes",
                    "hora_inicio": "10:00:00",
                    "hora_fin": "11:30:00",
                    "salon": "CD UP 13"
                },
                {
                    "dia": "Miércoles",
                    "hora_inicio": "10:00:00",
                    "hora_fin": "11:30:00",
                    "salon": "CD UP 07"
                }
            ]
        },
        {
            "materia": "Inteligencia Artificial",
            "profesor": "Hernández Uribe,Bernardo Irving",
            "schedules": [
                {
                    "dia": "Lunes",
                    "hora_inicio": "19:00:00",
                    "hora_fin": "20:30:00",
                    "salon": "Sin Asignar"
                },
                {
                    "dia": "Miércoles",
                    "hora_inicio": "19:00:00",
                    "hora_fin": "20:30:00",
                    "salon": "A2"
                }
            ]
        },
        {
            "materia": "Cálculo Vectorial",
            "profesor": "Ballesteros Flores,Fernando",
            "schedules": [
                {
                    "dia": "Lunes",
                    "hora_inicio": "08:30:00",
                    "hora_fin": "10:00:00",
                    "salon": "NR 05"
                },
                {
                    "dia": "Jueves",
                    "hora_inicio": "08:30:00",
                    "hora_fin": "10:00:00",
                    "salon": "JEREZ 01"
                },
                {
                    "dia": "Viernes",
                    "hora_inicio": "08:30:00",
                    "hora_fin": "10:00:00",
                    "salon": "NR 05"
                }
            ]
        },
        {
            "materia": "Gestión de la Calidad",
            "profesor": "Díaz Cantú,Rodrigo Eugenio",
            "schedules": [
                {
                    "dia": "Martes",
                    "hora_inicio": "16:00:00",
                    "hora_fin": "17:30:00",
                    "salon": "CDC_C"
                },
                {
                    "dia": "Jueves",
                    "hora_inicio": "16:00:00",
                    "hora_fin": "17:30:00",
                    "salon": "CDC_C"
                }
            ]
        },
        {
            "materia": "Taller de Diseño para la Sust.",
            "profesor": "Valencia Juárez,Itza",
            "schedules": [
                {
                    "dia": "Lunes",
                    "hora_inicio": "17:30:00",
                    "hora_fin": "18:30:00",
                    "salon": "R 02"
                },
                {
                    "dia": "Miércoles",
                    "hora_inicio": "17:30:00",
                    "hora_fin": "18:30:00",
                    "salon": "C 07"
                }
            ]
        },
        {
            "materia": "Dirección de Empresas",
            "profesor": "Villela Aranda,Fernando",
            "schedules": [
                {
                    "dia": "Miércoles",
                    "hora_inicio": "07:00:00",
                    "hora_fin": "09:00:00",
                    "salon": "D 07"
                }
            ]
        }
    ]
}

# Initialize an empty schedule dictionary
schedule_dict = {day: {} for day in ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes"]}

# Populate the schedule using new_Schedules
for course in new_Schedules["courses"]:
    for schedule in course["schedules"]:
        day = schedule["dia"]
        time_slot = f"{schedule['hora_inicio']} - {schedule['hora_fin']}"
        course_info = f"{course['materia']} ({course['profesor']})"
        if time_slot not in schedule_dict[day]:
            schedule_dict[day][time_slot] = course_info
        else:
            schedule_dict[day][time_slot] += f", {course_info}"

# Create a DataFrame to display the schedule
schedule_df = pd.DataFrame(schedule_dict).fillna("")

print("Horario Semanal")
print(schedule_df)

# Check for overlapping classes
overlap_issues = []

# Function to convert time string to pandas Timestamp
def to_timestamp(time_str):
    return pd.to_datetime(time_str, format="%H:%M:%S")

# Convert the schedules into a dictionary by days and times
for day, schedules in schedule_dict.items():
    time_slots = list(schedules.keys())
    for i in range(len(time_slots)):
        for j in range(i + 1, len(time_slots)):
            start_time_1_str, end_time_1_str = time_slots[i].split(" - ")
            start_time_2_str, end_time_2_str = time_slots[j].split(" - ")
            start_time_1 = to_timestamp(start_time_1_str)
            end_time_1 = to_timestamp(end_time_1_str)
            start_time_2 = to_timestamp(start_time_2_str)
            end_time_2 = to_timestamp(end_time_2_str)
            # Check for overlap
            if (start_time_1 < end_time_2) and (end_time_1 > start_time_2):
                overlap_issues.append({
                    "Dia": day,
                    "Hora Inicio 1": start_time_1.time(),
                    "Hora Fin 1": end_time_1.time(),
                    "Clase 1": schedules[time_slots[i]],
                    "Hora Inicio 2": start_time_2.time(),
                    "Hora Fin 2": end_time_2.time(),
                    "Clase 2": schedules[time_slots[j]]
                })

# Convert overlap issues to a DataFrame
overlap_df = pd.DataFrame(overlap_issues)

if not overlap_df.empty:
    print("\nConflictos de Horario")
    print(overlap_df)
else:
    print("\nNo hay conflictos de horario")
