#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashSet;
use serde::Serialize;

#[get("/hello")]
fn hello() -> String {
    format!("Hello_world")
}

#[get("/template")]
fn template() -> Template {

    let context = Context{};
    Template::render("index", &context)
}

#[derive(Serialize)]
struct Context {}

fn main() {
    println!("Hello, world!");

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![hello, template])
        .launch();
}
