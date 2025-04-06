use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FromRequest},
    response::Redirect,
    Request,
};
use rocket_dyn_templates::{context, Template};

use crate::database;

/// A user and their ID
pub struct User {
    pub id: u32,
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| database::get_user(id).ok())
            .or_forward(Status::Unauthorized)
    }
}

#[get("/login")]
pub async fn login_authenticated(user: User) -> Template {
    Template::render(
        "login",
        context! { username: user.name },
    )
}

#[get("/login", rank = 2)]
pub async fn login() -> Template {
    Template::render(
        "login",
        context! { },
    )
}

// TODO add logout button
#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private("user_id");
    Redirect::to(uri!(login))
}

#[derive(FromForm)]
pub struct LoginData<'r> {
    username: &'r str,
    password: &'r str,
    submit: &'r str,
}

#[post("/login", data = "<input>")]
pub async fn login_post(input: Form<LoginData<'_>>, jar: &CookieJar<'_>) -> Redirect {
    let user_res = match input.submit {
        "Register" => {
            database::register(input.username, input.password)
                .and_then(|_| database::login(input.username, input.password))
        }
        "Log In" => {
            database::login(input.username, input.password)
        }
        _ => panic!("invalid action")
    };
    if let Ok(user) = user_res {
        dbg!("login succeeded!");
        jar.add_private(Cookie::new("user_id", user.id.to_string()));
        Redirect::to(uri!(crate::home))
    } else {
        // TODO add error message if login fails
        dbg!("login failed!");
        Redirect::to(uri!(login))
    }
}

// #[post("/register", data = "<input>")]
// pub async fn register_post(input: Form<LoginData<'_>>, jar: &CookieJar<'_>) -> Redirect {
//     let user_result = database::register(input.username, input.password)
//         .and_then(|_| database::login(input.username, input.password));
//     if let Ok(user) = user_result {
//         dbg!("login succeeded!");
//         jar.add_private(Cookie::new("user_id", user.id.to_string()));
//         Redirect::to(uri!(crate::home))
//     } else {
//         // TODO add error message if login fails
//         dbg!("login failed!");
//         Redirect::to(uri!(login))
//     }
// }
