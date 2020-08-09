-- Your SQL goes here
CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name varchar(15) NOT NULL UNIQUE
)
