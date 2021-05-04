use rocket::response::Redirect;
use crate::login::Technician;
use crate::login::User;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct UserContext {
    user: User
}

#[get("/cameras", rank = 2)]
pub fn cameras_redirect_unauthorized() -> Redirect {
    Redirect::to(uri!(crate::views::login::login: "Neautorizuotas vartotojas"))
}

#[get("/cameras", rank = 1)]
pub fn cameras(technician: Technician) -> Template {
    let context = UserContext {
        user: technician.user
    };
    Template::render("cameras", &context)
}
