-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT IF EXISTS unique_email;
ALTER TABLE users DROP CONSTRAINT IF EXISTS unique_username;