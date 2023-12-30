-- Create a table to store tasks
CREATE TABLE IF NOT EXISTS tasks (
    task_id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date DATE,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- Inserting random values into the tasks table (up to 3 rows)
INSERT INTO tasks (title, description, due_date, completed, created_at, updated_at)
VALUES
  ('Task 1', 'Description for Task 1', '2023-01-15', FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('Task 2', 'Description for Task 2', '2023-02-28', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('Task 3', 'Description for Task 3', '2023-03-10', FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
  ;
