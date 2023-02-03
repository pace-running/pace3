-- Your SQL goes here
CREATE TABLE IF NOT EXISTS rejected_transactions (
    id SERIAL PRIMARY KEY,
    runner_ids VARCHAR NOT NULL,
    date_of_payment VARCHAR NOT NULL,
    reasons_for_payment VARCHAR NOT NULL,
    payment_amount VARCHAR NOT NULL,
    expected_amount VARCHAR,
    currency VARCHAR NOT NULL DEFAULT 'EUR',
    payer_name VARCHAR NOT NULL,
    iban VARCHAR NOT NULL
)