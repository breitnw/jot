use database::{post_note, Priority};
use rocket::{Build, Rocket};

mod database;

#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> String {
    let username = "burkalicious";
    let id = database::login(username, "pass2");
    let mut res = String::new();
    if let Ok(the_id) = id {
        res = res + &format!("USERNAME: {}\n", username);
        res = res + &format!("USER ID: {}\n", the_id);
        let my_notes = database::query_notes(the_id).unwrap();
        for note in my_notes {
            res = res
                + &format!(
                    "ID {}, dismissed: {}, priority: {}, [{}] {}\n",
                    note.post_id,
                    note.dismissed,
                    note.priority,
                    database::format_date(note.timestamp),
                    note.text,
                );
        }
    } else {
        res = res + "not yet registered!"
    }
    return res;
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

    rocket::build().mount("/", routes![home])
}
