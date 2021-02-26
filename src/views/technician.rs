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

#[get("/issues/<id>")]
pub fn issues_solve(id: i32) -> Template {
    let context = Context{
    };
    match id {
        1 => Template::render("technician/issues.history", &context),
        2 => Template::render("technician/issues.solve", &context),
        _ => Template::render("technician/issues.take", &context)
    }
}


#[get("/inventory")]
pub fn inventory() -> Template {
    let context = Context{};
    Template::render("technician/inventory", &context)
}

#[get("/inventory/register")]
pub fn inventory_register() -> Template {
    let context = Context{};
    Template::render("technician/inventory.register", &context)
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
