use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[get("/")]
pub fn index() -> Template {
    let context = Context{};
    Template::render("manager/index", &context)
}

#[get("/issues")]
pub fn issues() -> Template {
    let context = Context{};
    Template::render("manager/issues", &context)
}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = Context{};
    Template::render("manager/statistics", &context)
}
