use rocket_dyn_templates::{context, Template};
use crate::User;

#[get("/<username>/portfolio")]
pub fn get(username: String) -> Template {
    let user = User::from_token(username.as_str());
    match user {
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