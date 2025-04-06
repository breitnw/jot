use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, response::Redirect, Build, Rocket};
use rocket_dyn_templates::{context, Template};

// TODO bulletin?

pub mod auth;
pub mod database;

#[macro_use]
extern crate rocket;

#[get("/")]
fn home(user: auth::User) -> Template {
    Template::render(
        "home",
        context! {
            notes: database::query_notes(user.id).unwrap()
        },
    )
}

#[get("/", rank = 2)]
fn home_logged_out() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("inbox/static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    database::init().unwrap();

    // database::register("breitnw", "pass1").unwrap();
    // database::register("burkalicious", "pass2").unwrap();
    // database::post_note(
    //     2,
    //     "hi, this is my first post with a higher priority!",
    //     Priority::HIGH,
    // )
    // .unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                home,
                home_logged_out,
                auth::login,
                auth::login_post,
                auth::logout,
                static_file
            ],
        )
        .attach(Template::fairing())
}
