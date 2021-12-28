-- Your SQL goes here

ALTER TABLE payments
  ADD CONSTRAINT from_user_fk FOREIGN KEY (from_user) REFERENCES users (id);

ALTER TABLE payments
  ADD CONSTRAINT to_user_fk FOREIGN KEY (to_user) REFERENCES users (id);

ALTER TABLE transfers
  ADD CONSTRAINT for_user_fk FOREIGN KEY (for_user) REFERENCES users (id);
