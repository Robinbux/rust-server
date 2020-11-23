CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    todo_message TEXT NOT NULL,
    completed BOOLEAN NOT NULL
)