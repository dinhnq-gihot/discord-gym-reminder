-- Your SQL goes here
CREATE TABLE schedules (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES users(id),
    channel_id VARCHAR(50) NOT NULL,
    day VARCHAR(20) NOT NULL,
    start_time TIME NOT NULL
);