CREATE TABLE prizes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    stock INT NOT NULL DEFAULT 0
); 