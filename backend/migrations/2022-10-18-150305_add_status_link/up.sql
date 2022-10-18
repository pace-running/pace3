-- Your SQL goes here
CREATE OR REPLACE FUNCTION random_string(integer) RETURNS text AS $body$
    SELECT array_to_string(array(SELECT substring('ABCDEFGHIJKLMNOPQRSTUVWXYZ' FROM (ceil(random()*62))::int FOR 1) FROM generate_series(1, $1)), '');
$body$ LANGUAGE SQL VOLATILE;

ALTER TABLE runners
ADD COLUMN status_link VARCHAR NOT NULL DEFAULT CONCAT('https://pace3.lauf-gegen-rechts.de/status/', random_string(12));