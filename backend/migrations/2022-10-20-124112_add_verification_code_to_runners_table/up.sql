CREATE OR REPLACE FUNCTION random_string( int ) RETURNS TEXT as $body$
    SELECT string_agg(substring('0123456789abcdfghjkmnpqrstvwxyz', round(random() * 30)::integer, 1), '') FROM generate_series(1, $1);
$body$ LANGUAGE SQL VOLATILE;

ALTER TABLE runners
ADD COLUMN verification_code VARCHAR NOT NULL DEFAULT random_string(16);