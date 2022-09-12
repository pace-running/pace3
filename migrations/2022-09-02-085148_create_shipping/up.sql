-- Your SQL goes here
CREATE TABLE IF NOT EXISTS shippings (
    id SERIAL PRIMARY KEY,
    tshirt_model VARCHAR NOT NULL,
    tshirt_size VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    street_name VARCHAR NOT NULL,
    house_number VARCHAR NOT NULL,
    address_extra VARCHAR,
    postal_code VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    runner_id INTEGER NOT NULL,
    CONSTRAINT fk_runner FOREIGN KEY(runner_id) REFERENCES runners(id) ON DELETE SET NULL
)
