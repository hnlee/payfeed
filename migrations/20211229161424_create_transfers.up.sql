-- Add up migration script here

CREATE TABLE transfers (
  id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  amount decimal(19, 4) NOT NULL,
  for_user uuid NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  CONSTRAINT for_user_fk FOREIGN KEY (for_user) REFERENCES users (id)
);
