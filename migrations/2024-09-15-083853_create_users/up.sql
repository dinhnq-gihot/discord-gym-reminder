-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    name VARCHAR,
    point INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 0
);