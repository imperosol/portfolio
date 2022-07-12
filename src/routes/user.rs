use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use crate::model::User;

#[get("/portfolio/<username>")]
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

#[get("/portfolio/new")]
pub fn new() -> Template {
    Template::render("add_portfolio_form", context! {})
}

#[post("/portfolio/add", data = "<form>")]
pub fn add(form: Form<User>) -> Redirect {
    let res = form.create_json();
    match res {
        Ok(_) => {
            let username = format!("{}-{}", form.surname.to_lowercase(), form.name.to_lowercase());
            Redirect::to(uri!(get(username)))
        },
        Err(_) => Redirect::to(uri!(get("")))
    }
}

