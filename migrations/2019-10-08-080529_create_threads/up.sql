-- Your SQL goes here
CREATE TABLE threads (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  slug VARCHAR NOT NULL UNIQUE,
  title VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime')),
  updated_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);
