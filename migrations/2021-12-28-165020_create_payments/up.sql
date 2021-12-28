-- Your SQL goes here

CREATE TABLE payments (
  id uuid PRIMARY KEY NOT NULL,
  amount decimal(19, 4) NOT NULL,
  from_user uuid NOT NULL,
  to_user uuid NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL
);
