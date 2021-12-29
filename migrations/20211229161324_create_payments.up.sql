-- Add up migration script here

CREATE TABLE payments (
  id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  amount decimal(19, 4) NOT NULL,
  from_user uuid NOT NULL,
  to_user uuid NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  CONSTRAINT to_user_fk FOREIGN KEY (to_user) REFERENCES users (id),
  CONSTRAINT from_user_fk FOREIGN KEY (from_user) REFERENCES users (id)
);
