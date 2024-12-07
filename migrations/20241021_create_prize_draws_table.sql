CREATE TABLE prize_draws (
    id SERIAL PRIMARY KEY,
    prize_id INT NOT NULL,
    prize_name TEXT NOT NULL,
    user_id INT NOT NULL,
    user_name TEXT NOT NULL,
    department_name TEXT NOT NULL,
    ticket_number TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (prize_id) REFERENCES prizes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_prize_draws_prize_id ON prize_draws(prize_id);
CREATE INDEX idx_prize_draws_user_id ON prize_draws(user_id);
CREATE INDEX idx_prize_draws_created_at ON prize_draws(created_at); 