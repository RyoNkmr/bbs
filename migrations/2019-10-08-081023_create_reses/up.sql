-- Your SQL goes here

CREATE TABLE responses (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  body TEXT NOT NULL,
  ip VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime'))
);
