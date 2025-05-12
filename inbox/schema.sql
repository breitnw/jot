CREATE TABLE users (
  user_id INTEGER PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL
);

CREATE TABLE notes (
  note_id INTEGER PRIMARY KEY,
  text TEXT NOT NULL,
  timestamp BIGINT NOT NULL,
  dismissed BOOLEAN NOT NULL DEFAULT FALSE,
  priority TINYINT NOT NULL CHECK(priority >= 0 AND priority < 3) DEFAULT 0,
  user_id INTEGER NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users
);
