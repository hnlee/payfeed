-- This file should undo anything in `up.sql`

ALTER TABLE users
  ALTER COLUMN id DROP DEFAULT;

ALTER TABLE transfers
  ALTER COLUMN id DROP DEFAULT;

ALTER TABLE payments
  ALTER COLUMN id DROP DEFAULT;

DROP EXTENSION IF EXISTS "uuid-ossp";
