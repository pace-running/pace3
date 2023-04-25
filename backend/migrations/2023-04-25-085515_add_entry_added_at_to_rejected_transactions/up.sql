-- Your SQL goes here
ALTER TABLE rejected_transactions
    ADD COLUMN entry_added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT timezone('UTC', now());