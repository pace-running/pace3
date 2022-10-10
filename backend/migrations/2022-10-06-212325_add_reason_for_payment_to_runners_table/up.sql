CREATE OR REPLACE FUNCTION random_string(integer) RETURNS text AS $body$
    SELECT array_to_string(array(SELECT substring('ABCDEFGHIJKLMNOPQRSTUVWXYZ' FROM (ceil(random()*62))::int FOR 1) FROM generate_series(1, $1)), '');
$body$ LANGUAGE SQL VOLATILE;

ALTER TABLE runners
ADD COLUMN reason_for_payment VARCHAR NOT NULL DEFAULT CONCAT('LGR-', random_string(5));