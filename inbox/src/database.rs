use chrono::{DateTime, Utc};
use lib::{Note, Priority};
use rusqlite::Connection;

use crate::auth::User;

const DB_PATH: &'static str = "db.sqlite";

/// Set up the user and note database tables in data/db.sqlite
pub fn init() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        user_id INTEGER PRIMARY KEY,
        username TEXT UNIQUE NOT NULL,
        password TEXT NOT NULL)",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
        note_id INTEGER PRIMARY KEY,
        text TEXT NOT NULL,
        timestamp INTEGER NOT NULL,
        dismissed BOOLEAN NOT NULL DEFAULT FALSE,
        priority INTEGER NOT NULL CHECK(priority >= 0 AND priority < 3) DEFAULT 0,
        user_id INTEGER NOT NULL,
        FOREIGN KEY(user_id) REFERENCES users(id))",
        (),
    )?;
    Ok(())
}

/// Register a user, returning their ID
///
/// FIXME: does not currently hash users' passwords! Passwords are stored as
/// plain text in the `users` table.
pub fn register(username: &str, password: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        (username, password),
    )?;
    Ok(())
}

/// Attempt to log a user in given a username and password. If successful,
/// returns the user's ID.
pub fn login(username: &str, password: &str) -> Result<User, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(
        "SELECT user_id
         FROM users
         WHERE username = (?1) AND password = (?2)",
    )?;
    return stmt.query_row((username, password), |row| {
        Ok(User {
            id: row.get(0)?,
            name: username.to_owned(),
        })
    });
}

/// Get a user by their ID.
pub fn get_user(id: u32) -> Result<User, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(
        "SELECT username
         FROM users
         WHERE user_id = (?1)",
    )?;
    return stmt.query_row((id,), |row| {
        Ok(User {
            id,
            name: row.get(0)?,
        })
    });
}

/// Post a note encoded in `text` to a new entry in the database
pub fn post_note(user_id: u32, text: &str, priority: Priority) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "INSERT INTO notes (text, timestamp, user_id, priority)
         VALUES (?1, ?2, ?3, ?4)",
        (text, timestamp, user_id, priority as u32),
    )?;
    Ok(())
}

/// Dismiss the note with id `note-id`
pub fn dismiss_note(note_id: u32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "UPDATE notes
         SET dismissed = TRUE
         WHERE note_id = (?1)",
        (note_id,),
    )?;
    Ok(())
}

/// Query all of a user's notes, ordered by priority. Within each priority
/// level, notes are ordered by timestamp. Does not include dismissed notes.
pub fn query_notes(user_id: u32) -> Result<Vec<Note>, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(
        "SELECT note_id, text, timestamp, priority, dismissed
         FROM notes
         WHERE user_id = (?1) AND dismissed = FALSE
         ORDER BY priority DESC, timestamp DESC;",
    )?;
    let note_iter = stmt.query_map([user_id], |row| {
        Ok(Note {
            user_id,
            note_id: row.get(0)?,
            text: row.get(1)?,
            timestamp: format_date(row.get(2)?),
            priority: Priority::try_from(row.get::<_, u32>(3)?)
                .expect("Unexpected priority value!"),
            dismissed: row.get(4)?,
        })
    })?;
    return note_iter.collect();
}

/// Format a date timestamp as a human-readable string
pub fn format_date(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .format("%b %d, %Y | %H:%M")
        .to_string()
}
