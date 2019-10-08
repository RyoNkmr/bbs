-- Your SQL goes here
CREATE TABLE threads (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
  updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
);
