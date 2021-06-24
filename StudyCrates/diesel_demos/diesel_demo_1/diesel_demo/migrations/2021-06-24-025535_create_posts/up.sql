-- Your SQL goes here
CREATE TABLE posts (
  id INT PRIMARY KEY,
  title VARCHAR(45) NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
