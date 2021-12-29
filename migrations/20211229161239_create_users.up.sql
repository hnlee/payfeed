-- Add up migration script here

CREATE TABLE users (
  id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  name varchar(256) NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
