use chrono::prelude::*;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[get("/")]
pub fn index() -> Template {
    let context = Context{};
    Template::render("manager/super/index", &context)
}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = Context{};
    Template::render("manager/super/statistics", &context)

}
