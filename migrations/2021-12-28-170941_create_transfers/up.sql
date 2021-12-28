-- Your SQL goes here

CREATE TABLE transfers (
  id uuid PRIMARY KEY NOT NULL,
  amount decimal(19, 4) NOT NULL,
  for_user uuid NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
