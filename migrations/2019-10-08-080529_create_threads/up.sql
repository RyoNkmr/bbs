-- Your SQL goes here
CREATE TABLE threads (
  slug VARCHAR PRIMARY KEY UNIQUE NOT NULL,
  title VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime')),
  updated_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);
