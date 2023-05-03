-- This file should undo anything in `up.sql`
DROP TABLE registration_tokens;
ALTER TABLE users
DROP COLUMN registration_confirmed;