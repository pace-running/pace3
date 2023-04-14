-- This file should undo anything in `up.sql`
DELETE FROM theme
WHERE event_key IN ('is_registration_open','enable_tshirts','event_description');