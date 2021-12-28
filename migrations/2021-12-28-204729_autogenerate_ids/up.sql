-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE payments
  ALTER COLUMN id SET DEFAULT uuid_generate_v4();

ALTER TABLE transfers
  ALTER COLUMN id SET DEFAULT uuid_generate_v4();

ALTER TABLE users
  ALTER COLUMN id SET DEFAULT uuid_generate_v4();
