-- This file should undo anything in `up.sql`
ALTER TABLE shippings DROP CONSTRAINT fk_runner;
DROP TABLE shippings