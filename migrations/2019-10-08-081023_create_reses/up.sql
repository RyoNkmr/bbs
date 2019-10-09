-- Your SQL goes here

CREATE TABLE reses (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  thread_id INTEGER NOT NULL,
  user_name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  body TEXT NOT NULL,
  ip VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime')),
  FOREIGN KEY (thread_id) REFERENCES threads(id)
);
