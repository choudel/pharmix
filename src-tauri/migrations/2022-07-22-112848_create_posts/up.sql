-- Your SQL goes herediesel migration generate
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)