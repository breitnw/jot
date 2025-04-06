use core::fmt;
use std::fmt::write;

use chrono::{DateTime, Utc};
use rusqlite::Connection;

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
        post_id INTEGER PRIMARY KEY,
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
/// return the user's ID. If unsuccessful, return an error.
pub fn login(username: &str, password: &str) -> Result<u32, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt =
        conn.prepare("SELECT user_id FROM users WHERE username = (?1) AND password = (?2)")?;
    return stmt.query_row((username, password), |row| Ok(row.get(0)?));
}

/// Post a note encoded in `text` to a new entry in the database
pub fn post_note(user_id: u32, text: &str, priority: Priority) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "INSERT INTO notes (text, timestamp, user_id, priority) VALUES (?1, ?2, ?3, ?4)",
        (text, timestamp, user_id, priority as u32),
    )?;
    Ok(())
}

/// The priority assigned to a note: either low, medium, or high
pub enum Priority {
    LOW,
    MED,
    HIGH,
}

impl TryFrom<u32> for Priority {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Priority::LOW as u32 => Ok(Priority::LOW),
            x if x == Priority::MED as u32 => Ok(Priority::MED),
            x if x == Priority::HIGH as u32 => Ok(Priority::HIGH),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::HIGH => "HIGH",
                Self::MED => "MED",
                Self::LOW => "LOW",
            }
        )
    }
}

/// A single note and its metadata
pub struct Note {
    pub post_id: u32,
    pub user_id: u32,
    pub text: String,
    pub timestamp: i64,
    pub priority: Priority,
    pub dismissed: bool,
}

/// Query all of a user's notes, with the most recent note first
pub fn query_notes(user_id: u32) -> Result<Vec<Note>, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(
        "SELECT post_id, text, timestamp, priority, dismissed
         FROM notes
         WHERE user_id = (?1)
         ORDER BY timestamp DESC",
    )?;
    let note_iter = stmt.query_map([user_id], |row| {
        Ok(Note {
            user_id,
            post_id: row.get(0)?,
            text: row.get(1)?,
            timestamp: row.get(2)?,
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
        .format("%d/%m/%Y %H:%M")
        .to_string()
}
