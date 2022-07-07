mod model;

#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer};
use rocket_dyn_templates::{context, Template};
use rocket::response::content::RawHtml;
use crate::model::User;


#[get("/")]
fn index() -> RawHtml<String> {
    let links = User::get_all_names()
        .iter()
        .map(|user| format!("<p><a href=\"{}\">{} {}</a></p>",
                            User::url_alias(user), user[0], user[1]))
        .collect::<Vec<String>>()
        .join("");
    RawHtml(links)
}

#[get("/pd")]
fn pd() -> &'static str {
    "Gros pd"
}

#[get("/portfolio/<username>")]
fn get_user(username: String) -> Template {
    let json = User::get_json(username.as_str());
    match json {
        None => {
            Template::render("user_page", context! {
                user: User::empty()
            })
        }
        Some(user) => {
            Template::render("user_page", context! {
                user: user
            })
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, pd, get_user])
        .mount("/public", FileServer::from("./static/public").rank(9))
        .mount("/", FileServer::from("./templates"))
        .attach(Template::fairing())
}

