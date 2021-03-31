use rocket_contrib::templates::Template;
use crate::MaintenanceDb;
use rocket::response::Redirect;
use rocket::request::Form;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[get("/login")]
pub fn login() -> Template {
    let context = Context {};
    Template::render("login", &context)
}

#[derive(Debug, FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data="<user_data>")]
pub fn login_action(user_data: Form<LoginForm>, db: MaintenanceDb)
                    -> Template {
    println!("user_data: {:?}", user_data);

    let context = Context {};
    Template::render("technician/index", &context)
}
