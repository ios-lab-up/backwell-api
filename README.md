# 🏫 Backwell API 🚀

**Backwell API** is a robust application designed to create optimal schedule combinations for students by leveraging two powerful microservices: a Rust-based scheduling engine and a Django backend for managing courses and instructors.

![Backwell API Banner](https://via.placeholder.com/1200x300.png?text=Backwell+API+Banner)

## 📖 Table of Contents

- [🌟 Features](#-features)
- [📦 Technologies Used](#-technologies-used)
- [🔧 Installation](#-installation)
  - [Prerequisites](#prerequisites)
  - [Steps](#steps)
- [🚀 Running the Application](#-running-the-application)
- [🛠️ Services Overview](#️-services-overview)
  - [🔨 Rust App - Backwell API](#-rust-app---backwell-api)
  - [🐍 Django Backend](#-django-backend)
- [📚 API Endpoints](#-api-endpoints)
  - [Generate Schedule](#generate-schedule)
  - [Course Management](#course-management)
- [📄 Example Usage](#-example-usage)
- [📝 Contributing](#-contributing)
- [📜 License](#-license)
- [📫 Contact](#-contact)

## 🌟 Features

- **Microservices Architecture**: Separates scheduling logic and course management for scalability.
- **Rust Performance**: Utilizes Rust's `actix-web` framework for high-performance API operations.
- **Django Backend**: Manages courses, professors, subjects, and classrooms efficiently.
- **Dockerized Setup**: Simplifies deployment and environment consistency.
- **JSON-Based API**: Easy integration with other systems and services.

## 📦 Technologies Used

- **Rust** 🦀
- **Django** 🐍
- **Docker** 🐳
- **Actix-web** 🔗
- **PostgreSQL** 🐘

## 🔧 Installation

### Prerequisites

Before getting started, ensure you have the following installed on your system:

- [Git](https://git-scm.com/) - Version control system
- [Docker](https://www.docker.com/get-started) - Containerization platform
- [Docker Compose](https://docs.docker.com/compose/install/) - Tool for defining and running multi-container Docker applications

### Steps

1. **Clone the Repository**

   ```bash
   git clone https://github.com/your-username/backwell-api.git
   cd backwell-api
   ```

2. **Download Docker**

   If you haven't installed Docker yet, follow the [official Docker installation guide](https://docs.docker.com/get-docker/) for your operating system.

3. **Build and Run the Containers**

   ```bash
   docker-compose up --build
   ```

   This command will build the Docker images and start the containers for both the Rust API and Django backend.

4. **Verify Services are Running**

   Ensure that the following services are up and running:

   - **Web-1 (Django Backend)**
   - **DB-1 (PostgreSQL Database)**
   - **Rust App (Backwell API)**

   You can verify by checking the Docker containers:

   ```bash
   docker ps
   ```

5. **Access the Services**

   - **Django Backend**: [http://localhost:8001](http://localhost:8001)
   - **Rust API**: [https://localhost:8082](https://localhost:8082)

## 🚀 Running the Application

Once the Docker containers are up and running, the microservices can be accessed via the following ports:

- **Django Backend API**: `http://localhost:8001`
- **Rust Scheduling API**: `https://localhost:8082`

You can interact with the APIs using tools like [Postman](https://www.postman.com/) or [cURL](https://curl.se/).

## 🛠️ Services Overview

### 🔨 Rust App — Backwell API

**Backwell API** is built using Rust and the `actix-web` framework. It is responsible for generating class schedules based on available courses and professors.

- **Endpoint**: `/generate_schedule`
- **Method**: `POST`
- **Purpose**: Generates compatible schedule groupings based on the provided course data.

**Key Features:**

- High-performance scheduling algorithms
- Secure communication with the Django backend
- JSON-based request and response handling

### 🐍 Django Backend

The Django backend manages the core data for the application, including courses, professors, subjects, and classrooms. It provides RESTful endpoints for data retrieval and management.

**Key Features:**

- CRUD operations for courses, professors, subjects, and classrooms
- Admin interface for easy data management
- Integration with the Rust API for scheduling

## 📚 API Endpoints

### Generate Schedule

- **URL**: `https://localhost:8082/v1/api/generate_schedule`
- **Method**: `POST`
- **Description**: Generates schedule combinations based on selected courses.

**Request Body Example:**

```json
{
  "courses": ["Ética", "Persona y sociedad"],
  "minimum": 1
}
```

### Course Management

The Django backend provides several endpoints to manage courses and related entities:

- **Courses**: `http://localhost:8001/api/cursos`
- **Professors**: `http://localhost:8001/api/profesores`
- **Subjects**: `http://localhost:8001/api/materias`
- **Classrooms**: `http://localhost:8001/api/salones`

Each endpoint supports `GET`, `POST`, `PUT`, and `DELETE` methods for comprehensive data management.

## 📄 Example Usage

### Generating a Schedule

Use the following `cURL` command to generate a schedule:

```bash
curl -X POST https://localhost:8082/v1/api/generate_schedule \
     -H "Content-Type: application/json" \
     -d '{
           "courses": ["Mathematics", "Physics"],
           "minimum": 2
         }'
```

**Expected Response:**

```json
{
  "schedule_groups": [
    [
      {
        "id": 1,
        "materia": {"id": 101, "codigo": "MATH101", "nombre": "Mathematics", "no_de_catalogo": "101"},
        "profesor": {"id": 5, "nombre": "Dr. Smith"},
        "salon": {"id": 10, "nombre": "Room A", "capacidad": 30},
        "hora_inicio": "08:00",
        "hora_fin": "10:00",
        "lunes": true,
        "martes": false,
        "miércoles": true,
        "jueves": false,
        "viernes": true
      },
      ...
    ],
    ...
  ]
}
```

This response provides groups of compatible course schedules, ensuring no conflicts in timings, instructors, or classrooms.

## 📝 Contributing

Contributions are welcome! Please follow these steps to contribute:

1. **Fork the Repository**

2. **Create a New Branch**

   ```bash
   git checkout -b feature/YourFeatureName
   ```

3. **Commit Your Changes**

   ```bash
   git commit -m "Add some feature"
   ```

4. **Push to the Branch**

   ```bash
   git push origin feature/YourFeatureName
   ```

5. **Open a Pull Request**

Please ensure your code follows the project's coding standards and includes appropriate tests.

## 📜 License

This project is licensed under the [MIT License](LICENSE).

## 📫 Contact

For any inquiries or support, please contact:

- **Email**: [isiguenza@up.edu.mx](mailto:isiguenza@up.edu.mx)
- **GitHub**: [@JavierRangel2004](https://github.com/JavierRangel2004)

---

✨ *Empowering students with optimized scheduling solutions!* ✨