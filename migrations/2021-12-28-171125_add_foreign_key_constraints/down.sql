-- This file should undo anything in `up.sql`

ALTER TABLE payments DROP CONSTRAINT from_user_fk;
ALTER TABLE payments DROP CONSTRAINT to_user_fk;
ALTER TABLE transfers DROP CONSTRAINT for_user_fk;
