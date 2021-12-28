-- Your SQL goes here

CREATE TABLE users (
  id uuid PRIMARY KEY NOT NULL,
  name varchar(256) NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
