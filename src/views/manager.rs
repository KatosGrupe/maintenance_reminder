use chrono::prelude::*;
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

#[derive(Serialize)]
struct IssuesRegisterContext {
    current_datetime: String
}

#[get("/issues/register")]
pub fn issues_register() -> Template {
    let context = IssuesRegisterContext{
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
    };
    Template::render("manager/issues.register", &context)
}

//TODO: fix to use decent type and not string
#[derive(Serialize)]
struct StatisticsContext {
    from_date: String,
    to_date: String
}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = StatisticsContext{
        from_date: "2021-01-24".to_string(),
        to_date: "2021-02-24".to_string(),
    };
    Template::render("manager/statistics", &context)
}

#[get("/settings")]
pub fn settings() -> Template {
    let context = Context{};
    Template::render("manager/settings", &context)
}
