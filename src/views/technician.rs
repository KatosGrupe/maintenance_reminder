use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[get("/")]
pub fn index() -> Template {
    let context = Context{};
    Template::render("technician/index", &context)
}

#[get("/issues")]
pub fn issues() -> Template {
    let context = Context{};
    Template::render("technician/issues", &context)
}

#[get("/issues/register")]
pub fn issues_register() -> Template {
    let context = Context{};
    Template::render("technician/issues.register", &context)
}


#[get("/inventory")]
pub fn inventory() -> Template {
    let context = Context{};
    Template::render("technician/inventory", &context)
}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = Context{};
    Template::render("technician/statistics", &context)
}

#[get("/settings")]
pub fn settings() -> Template {
    let context = Context{};
    Template::render("technician/settings", &context)
}
