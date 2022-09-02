-- Your SQL goes here
CREATE TABLE IF NOT EXISTS runners (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR,
    lastname VARCHAR,
    team VARCHAR,
    email VARCHAR,
    starting_point VARCHAR NOT NULL,
    running_level VARCHAR NOT NULL,
    donation VARCHAR NOT NULL
)