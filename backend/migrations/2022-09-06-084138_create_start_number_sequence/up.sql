-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS runner_start_number_seq
INCREMENT 1
START 4
OWNED BY runners.start_number;
