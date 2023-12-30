# TODO API

## Overview
This documentation describes the endpoints and methods for the TODO API.

## API Information
- **API Reference**: See project [models.rs](/backend/src/models.rs) for the API reference.

## Endpoints

### 1. Get Tasks

- **Endpoint:** `GET /todo`
- **Description:** Retrieve a list of tasks.
- **Request:**
  ```http
  GET http://127.0.0.1:80/todo
  ```
- **Response:** 
```json
[
  {
    "completed": false,
    "created_at": "2023-12-30T16:45:11.142846",
    "description": "Description for Task 1",
    "due_date": "2023-01-15",
    "task_id": 1,
    "title": "Task 1",
    "updated_at": "2023-12-30T16:45:11.142846"
  },
  {
    "completed": true,
    "created_at": "2023-12-30T16:45:11.142846",
    "description": "Description for Task 2",
    "due_date": "2023-02-28",
    "task_id": 2,
    "title": "Task 2",
    "updated_at": "2023-12-30T16:45:11.142846"
  },
  {
    "completed": false,
    "created_at": "2023-12-30T16:45:11.142846",
    "description": "Description for Task 3",
    "due_date": "2023-03-10",
    "task_id": 3,
    "title": "Task 3",
    "updated_at": "2023-12-30T16:45:11.142846"
  }
]
```

### 2. Delete Task

- **Endpoint:** `DELETE /todo/{taskId}`
- **Description:** Delete a specific task.
- **Request:**
  ```http
  DELETE http://127.0.0.1:80/todo/1
  ```
- **Response:** (No specific response details provided)

### 3. Add Task

- **Endpoint:** `PUT /todo`
- **Description:** Add a new task.
- **Request:**
  ```http
  PUT http://127.0.0.1:80/todo
  ```
- **Request Body:**
  ```json
  {
      "completed": true,
      "description": "This task has been already done",
      "title": "This is an old task, which has already been finished"
  }
  ```
- **Response:** (No specific response details provided)

### 4. Update Task

- **Endpoint:** `PATCH /todo/{taskId}`
- **Description:** Update details of a specific task.
- **Request:**
  ```http
  PATCH http://127.0.0.1:80/todo/7
  ```
- **Request Body:**
  ```json
  {
      "completed": false,
      "description": "This task has not quite been done.",
      "title": "This is an old task, which is yet unsolved."
  }
  ```
- **Response:** (No specific response details provided)