DROP TABLE IF EXISTS user_tickets CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS team CASCADE;

CREATE TABLE team (
    id SERIAL PRIMARY KEY,
    team_name TEXT NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    team_id INT,
    role VARCHAR(255) NOT NULL DEFAULT 'user',
    FOREIGN KEY (team_id) REFERENCES team(id) ON DELETE SET NULL
);

CREATE TABLE user_tickets (
    id SERIAL PRIMARY KEY,
    user_id INT,
    ticket_number TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
