mod model;
mod routes;

#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer};
use rocket_dyn_templates::{Template};
use rocket::response::content::RawHtml;
use model::User;
use routes::user;


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



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, user::get])
        .mount("/public", FileServer::from("./static/public").rank(9))
        .mount("/", FileServer::from("./templates"))
        .attach(Template::fairing())
}

