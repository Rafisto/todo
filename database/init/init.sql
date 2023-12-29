-- Create a table to store users
CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(100) NOT NULL
);

-- Create a table to store tasks
CREATE TABLE IF NOT EXISTS tasks (
    task_id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(user_id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date DATE,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Inserting admin values into the users table (1 row)
INSERT INTO users (username, password_hash) VALUES ('admin', '8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918');

-- Inserting random values into the tasks table (up to 3 rows)
INSERT INTO tasks (user_id, title, description, due_date, completed, created_at, updated_at)
VALUES
  (1, 'Task 1', 'Description for Task 1', '2023-01-15', FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  (1, 'Task 2', 'Description for Task 2', '2023-02-28', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  (1, 'Task 3', 'Description for Task 3', '2023-03-10', FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
  ;
