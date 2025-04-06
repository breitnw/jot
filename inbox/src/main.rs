use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, response::Redirect, serde::json::Json, Build, Rocket};
use rocket_dyn_templates::{context, Template};

// TODO bulletin?

pub mod auth;
pub mod database;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn home_authenticated(user: auth::User) -> Template {
    Template::render(
        "home",
        context! {
            username: user.name,
            notes: database::query_notes(user.id).unwrap(),
        },
    )
}

#[get("/", rank = 2)]
async fn home() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("inbox/static/").join(file)).await.ok()
}

#[post("/dismiss/<note_id>")]
async fn dismiss(note_id: u32) {
    database::dismiss_note(note_id).unwrap();
}

#[post("/post", data = "<input>")]
async fn post(input: Json<lib::NotePost>) {
    database::post_note(input.user_id, &input.text, input.priority).unwrap();
}

#[launch]
fn rocket() -> Rocket<Build> {
    database::init().unwrap();

    // use lib::Priority;
    // database::post_note(
    //     2,
    //     "this is my first post with a medium priority!",
    //     Priority::MED,
    // )
    // .unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                home_authenticated,
                home,
                auth::login_authenticated,
                auth::login,
                auth::login_post,
                auth::logout,
                static_file,
                dismiss,
                post,
            ],
        )
        .attach(Template::fairing())
}
