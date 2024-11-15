import pandas as pd

# JSON data
courses = [
    {"materia": "Química", "profesor": "Reyes Rodríguez,Juana Elisa", 
     "schedules": [{"dia": "Martes", "hora_inicio": "11:30:00", "hora_fin": "13:00:00"},
                   {"dia": "Lunes", "hora_inicio": "14:30:00", "hora_fin": "16:00:00"}]},
    {"materia": "Álgebra", "profesor": "Valtierra Quintal,Zeus Alberto", 
     "schedules": [{"dia": "Martes", "hora_inicio": "14:30:00", "hora_fin": "16:00:00"},
                   {"dia": "Jueves", "hora_inicio": "14:30:00", "hora_fin": "16:00:00"},
                   {"dia": "Viernes", "hora_inicio": "13:00:00", "hora_fin": "14:30:00"}]},
    {"materia": "Química", "profesor": "González Reyes,Estela", 
     "schedules": [{"dia": "Martes", "hora_inicio": "13:00:00", "hora_fin": "14:30:00"},
                   {"dia": "Jueves", "hora_inicio": "13:00:00", "hora_fin": "14:30:00"}]},
    {"materia": "Inteligencia Artificial", "profesor": "Hernández Uribe,Bernardo Irving", 
     "schedules": [{"dia": "Lunes", "hora_inicio": "19:00:00", "hora_fin": "20:30:00"},
                   {"dia": "Miércoles", "hora_inicio": "19:00:00", "hora_fin": "20:30:00"}]},
    {"materia": "Álgebra", "profesor": "Bernabe López,Eduardo", 
     "schedules": [{"dia": "Lunes", "hora_inicio": "11:30:00", "hora_fin": "13:00:00"},
                   {"dia": "Miércoles", "hora_inicio": "11:30:00", "hora_fin": "13:00:00"},
                   {"dia": "Viernes", "hora_inicio": "11:30:00", "hora_fin": "13:00:00"}]},
    {"materia": "Química", "profesor": "Salazar Hernández,Jenifer", 
     "schedules": [{"dia": "Martes", "hora_inicio": "07:00:00", "hora_fin": "08:30:00"},
                   {"dia": "Jueves", "hora_inicio": "07:00:00", "hora_fin": "08:30:00"}]},
    {"materia": "Álgebra", "profesor": "Montes Isunza,Sebastián", 
     "schedules": [{"dia": "Martes", "hora_inicio": "08:30:00", "hora_fin": "10:00:00"},
                   {"dia": "Miércoles", "hora_inicio": "08:30:00", "hora_fin": "10:00:00"},
                   {"dia": "Viernes", "hora_inicio": "08:30:00", "hora_fin": "10:00:00"}]},
    {"materia": "Química", "profesor": "Torres Luna,Verónica", 
     "schedules": [{"dia": "Martes", "hora_inicio": "16:00:00", "hora_fin": "17:30:00"},
                   {"dia": "Jueves", "hora_inicio": "16:00:00", "hora_fin": "17:30:00"}]},
    {"materia": "Mecánica", "profesor": "Reyes Razo,Ricardo", 
     "schedules": [{"dia": "Lunes", "hora_inicio": "10:00:00", "hora_fin": "11:30:00"},
                   {"dia": "Miércoles", "hora_inicio": "10:00:00", "hora_fin": "11:30:00"}]}
]

# Initialize an empty schedule dictionary
schedule_dict = {day: {} for day in ["Lunes", "Martes", "Miércoles", "Jueves", "Viernes"]}

# Populate the schedule
for course in courses:
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

# Convert the schedules into a dictionary by days and times
for day, schedules in schedule_dict.items():
    time_slots = list(schedules.keys())
    for i in range(len(time_slots)):
        for j in range(i + 1, len(time_slots)):
            start_time_1, end_time_1 = map(pd.Timestamp, time_slots[i].split(" - "))
            start_time_2, end_time_2 = map(pd.Timestamp, time_slots[j].split(" - "))
            if (start_time_1 < end_time_2 and end_time_1 > start_time_2):
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
    print("Conflictos de Horario")
    print(overlap_df)
else:
    print("No hay conflictos de horario")
