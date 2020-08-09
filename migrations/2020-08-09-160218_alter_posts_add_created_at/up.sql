-- Your SQL goes here
ALTER TABLE posts
    ADD COLUMN IF NOT EXISTS created_at timestamp with time zone NOT NULL DEFAULT now()
