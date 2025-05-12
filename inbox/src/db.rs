use chrono::{DateTime, Utc};
use lib::{Note, Priority};

use rocket::futures::{FutureExt, StreamExt, TryStreamExt};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

use crate::auth::User;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);

// const DB_PATH: &'static str = "db.sqlite";

// /// Set up the user and note database tables in data/db.sqlite
// pub fn init() -> Result<(), rusqlite::Error> {
//     let conn = Connection::open(DB_PATH)?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS users (
//         user_id INTEGER PRIMARY KEY,
//         username TEXT UNIQUE NOT NULL,
//         password TEXT NOT NULL)",
//         (),
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS notes (
//         note_id INTEGER PRIMARY KEY,
//         text TEXT NOT NULL,
//         timestamp INTEGER NOT NULL,
//         dismissed BOOLEAN NOT NULL DEFAULT FALSE,
//         priority INTEGER NOT NULL CHECK(priority >= 0 AND priority < 3) DEFAULT 0,
//         user_id INTEGER NOT NULL,
//         FOREIGN KEY(user_id) REFERENCES users(id))",
//         (),
//     )?;
//     Ok(())
// }

/// Register a user, returning their ID
///
/// FIXME: does not currently hash users' passwords! Passwords are stored as
/// plain text in the `users` table.
pub async fn register(
    db: &mut Connection<DB>,
    username: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
        .bind(username)
        .bind(password)
        .execute(&mut ***db)
        .await?;
    Ok(())
}

/// Attempt to log a user in given a username and password. If successful,
/// returns the user's ID.
pub async fn login(
    db: &mut Connection<DB>,
    username: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let q = sqlx::query(
        "SELECT user_id
         FROM users
         WHERE username = (?1) AND password = (?2)",
    );
    return q
        .bind(username)
        .bind(password)
        .map(|row| User {
            id: row.get(0),
            name: username.to_owned(),
        })
        .fetch_one(&mut ***db)
        .await;
}

/// Get a user by their ID.
pub async fn get_user(
    db: &mut Connection<DB>,
    id: u32,
) -> Result<User, sqlx::Error> {
    let q = sqlx::query(
        "SELECT username
         FROM users
         WHERE user_id = (?1)",
    );
    return q
        .bind(id)
        .map(|row| User {
            id,
            name: row.get(0),
        })
        .fetch_one(&mut ***db)
        .await;
}

/// Post a note encoded in `text` to a new entry in the database
pub async fn post_note(
    db: &mut Connection<DB>,
    user_id: u32,
    text: &str,
    priority: Priority,
) -> Result<(), sqlx::Error> {
    let timestamp = Utc::now().timestamp();
    let q = sqlx::query(
        "INSERT INTO notes (text, timestamp, user_id, priority)
         VALUES (?1, ?2, ?3, ?4)",
    );
    q.bind(text)
        .bind(timestamp)
        .bind(user_id)
        .bind(priority as u8)
        .execute(&mut ***db)
        .await?;
    Ok(())
}

/// Dismiss the note with id `note-id`, asserting that the corresponding
/// user has `user_id`
pub async fn dismiss_note(
    db: &mut Connection<DB>,
    note_id: u32,
    user_id: u32,
) -> Result<(), sqlx::Error> {
    let q = sqlx::query(
        "UPDATE notes
         SET dismissed = TRUE
         WHERE note_id = (?1) AND user_id = (?2)",
    );
    q.bind(note_id).bind(user_id).execute(&mut ***db).await?;
    Ok(())
}

/// Query all of a user's notes, ordered by priority. Within each priority
/// level, notes are ordered by timestamp. Does not include dismissed notes.
pub async fn query_notes(
    db: &mut Connection<DB>,
    user_id: u32,
) -> Result<Vec<Note>, sqlx::Error> {
    let q = sqlx::query(
        "SELECT note_id, text, timestamp, priority, dismissed
         FROM notes
         WHERE user_id = (?1) AND dismissed = FALSE
         ORDER BY priority DESC, timestamp DESC;",
    );
    return q
        .bind(user_id)
        .map(|row| Note {
            user_id,
            note_id: row.get(0),
            text: row.get(1),
            timestamp: format_date(row.get(2)),
            priority: Priority::try_from(row.get::<u8, _>(3))
                .expect("Unexpected priority value!"),
            dismissed: row.get(4),
        })
        .fetch_all(&mut ***db)
        .await;
}

/// Format a date timestamp as a human-readable string
pub fn format_date(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .format("%b %d, %Y | %H:%M")
        .to_string()
}
