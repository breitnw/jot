// THINGS TO DO
// TODO support adding notes from web dashboard
// TODO support viewing archived notes
// TODO animations for dismissing notes
// TODO password hashing
// TODO better security for adding notes
// TODO favicon
// TODO fix style for login form

use std::path::{Path, PathBuf};

use rocket::{
    fs::NamedFile, request::FlashMessage, response::Redirect,
    serde::json::Json, Build, Rocket,
};
use rocket_dyn_templates::{context, Template};

pub mod auth;
pub mod db;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn home(user: auth::User, error: Option<FlashMessage<'_>>) -> Template {
    Template::render(
        "home",
        context! {
            user: Some(&user),
            error: error,
            notes: db::query_notes(user.id).unwrap(),
        },
    )
}

#[get("/", rank = 2)]
async fn login_redirect() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("inbox/static/").join(file))
        .await
        .ok()
}

// FIXME make this secure - check cookie to see that the user is logged in to
// the right account
#[post("/dismiss/<note_id>")]
async fn dismiss(user: auth::User, note_id: u64) {
    db::dismiss_note(note_id, user.id).unwrap();
}

// FIXME is there any way to make this secure?
#[post("/post", data = "<input>")]
async fn post(input: Json<lib::NoteRequest>) {
    db::post_note(input.user_id, &input.text, input.priority).unwrap();
}

#[launch]
fn rocket() -> Rocket<Build> {
    db::init().unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                home,
                login_redirect,
                auth::login,
                auth::login_post,
                auth::register,
                auth::register_post,
                auth::logout,
                static_file,
                dismiss,
                post,
            ],
        )
        .attach(Template::fairing())
}
