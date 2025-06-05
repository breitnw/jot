// THINGS TO DO

// IMPORTANT STUFF
// TODO support viewing archived notes (send all to client, checkboxes toggle)
// TODO password hashing
// TODO priority selection from web dashboard
// TODO properly deploy with tokens and stuff, don't share Rocket.toml on git

// POLISH (HARDER)
// TODO custom themes?
// TODO animations for adding and dismissing notes
// TODO see if there's a more efficient way to add/remove notes without
//      re-fetching everything

// POLISH (EASIER)
// TODO "you're all caught up" message
// TODO enter to submit notes
// TODO error messages when can't connect to server
// TODO loading spinner
// TODO you should actually be able to get to the inbox from the homepage

use std::path::{Path, PathBuf};

use db::DB;
use lib::Note;
use rocket::{
    fs::NamedFile, request::FlashMessage, response::Redirect,
    serde::json::Json, Build, Rocket,
};
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::{context, Template};

pub mod auth;
pub mod db;

#[macro_use]
extern crate rocket;

// USER-FACING ENDPOINTS =======================================================

#[get("/")]
async fn inbox(user: auth::User, error: Option<FlashMessage<'_>>) -> Template {
    Template::render(
        "inbox",
        context! {
            user: Some(&user),
            error: error,
        },
    )
}

#[get("/", rank = 2)]
async fn landing(user: Option<auth::User>) -> Template {
    Template::render(
        "landing",
        context! {
            user: user
        },
    )
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("inbox/static/").join(file))
        .await
        .ok()
}

// API ENDPOINTS ===============================================================

#[post("/api/dismiss/<note_id>")]
async fn dismiss(mut db: Connection<DB>, user: auth::User, note_id: u32) {
    db::dismiss_note(&mut db, note_id, user.id).await.unwrap();
}

#[post("/api/post", data = "<input>")]
async fn post(
    mut db: Connection<DB>,
    user: auth::User,
    input: Json<lib::NoteRequest>,
) {
    db::post_note(&mut db, user.id, &input.text, input.priority)
        .await
        .unwrap();
}

#[get("/api/notes")]
async fn fetch(mut db: Connection<DB>, user: auth::User) -> Json<Vec<Note>> {
    // FIXME error 500 instead of unwrap
    Json(db::query_notes(&mut db, user.id).await.unwrap())
}

// LAUNCH ======================================================================

#[launch]
fn rocket() -> Rocket<Build> {
    // db::init().unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                // basic
                landing,
                inbox,
                static_file,
                // authentication
                auth::login,
                auth::login_post,
                auth::register,
                auth::register_post,
                auth::logout,
                // API
                dismiss,
                post,
                fetch
            ],
        )
        .attach(Template::fairing())
        .attach(DB::init())
}
