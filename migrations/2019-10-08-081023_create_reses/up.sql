-- Your SQL goes here

CREATE TABLE reses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  thread_id INTEGER NOT NULL,
  username VARCHAR NOT NULL,
  userid VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  body TEXT NOT NULL,
  ip VARCHAR NOT NULL,
  created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
  FOREIGN KEY (thread_id) REFERENCES threads(id),
);
