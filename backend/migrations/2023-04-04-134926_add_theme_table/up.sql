-- Your SQL goes here
CREATE TABLE IF NOT EXISTS theme (
    event_key VARCHAR NOT NULL PRIMARY KEY,
    event_value VARCHAR NOT NULL
);

INSERT INTO theme (event_key, event_value) VALUES ('event_name', 'Lauf gegen Rechts') ON CONFLICT DO NOTHING;