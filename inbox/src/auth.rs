use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FlashMessage, FromRequest},
    response::{Flash, Redirect},
    serde::{Deserialize, Serialize},
    Request,
};
use rocket_dyn_templates::{context, Template};

use crate::db;

/// A user and their ID
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(
        req: &'r Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| db::get_user(id).ok())
            .or_forward(Status::Unauthorized)
    }
}

#[get("/login")]
pub async fn login(
    user: Option<User>,
    error: Option<FlashMessage<'_>>,
) -> Template {
    Template::render("login", context! { user: user, error: error })
}

#[get("/register")]
pub async fn register(
    user: Option<User>,
    error: Option<FlashMessage<'_>>,
) -> Template {
    Template::render("register", context! { user: user, error: error })
}

#[derive(FromForm)]
pub struct LoginData<'r> {
    username: &'r str,
    password: &'r str,
}

#[post("/login", data = "<input>")]
pub async fn login_post(
    input: Form<LoginData<'_>>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Flash<Redirect>> {
    let user_res = db::login(input.username, input.password);
    if let Ok(user) = user_res {
        jar.add_private(Cookie::new("user_id", user.id.to_string()));
        Ok(Redirect::to(uri!(crate::home)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login)),
            "invalid username or password",
        ))
    }
}

#[derive(FromForm)]
pub struct RegisterData<'r> {
    username: &'r str,
    password: &'r str,
    password_confirm: &'r str,
}

#[post("/register", data = "<input>")]
pub async fn register_post(
    input: Form<RegisterData<'_>>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Flash<Redirect>> {
    // ensure password and confirmation password match
    if input.password != input.password_confirm {
        return Err(Flash::error(
            Redirect::to(uri!(register)),
            "passwords do not match",
        ));
    }
    // ensure password is long enough
    if input.password.len() < 8 {
        return Err(Flash::error(
            Redirect::to(uri!(register)),
            "password must be 8 characters or longer",
        ));
    }
    // register the user
    let register_res = db::register(input.username, input.password);
    if register_res.is_err() {
        return Err(Flash::error(
            Redirect::to(uri!(register)),
            &format!("username \"{}\" is taken", input.username),
        ));
    }
    // log the user in
    let user_res = db::login(input.username, input.password);
    if let Ok(user) = user_res {
        jar.add_private(Cookie::new("user_id", user.id.to_string()));
        Ok(Redirect::to(uri!(crate::home)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login)),
            "invalid username or password",
        ))
    }
}

#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private("user_id");
    Redirect::to(uri!(login))
}
