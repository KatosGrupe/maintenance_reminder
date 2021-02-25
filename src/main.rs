#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use serde::Serialize;
use rocket_contrib::serve::StaticFiles;

pub mod views;

#[get("/hello")]
fn hello() -> String {
    format!("Hello_world")
}

#[derive(Serialize)]
struct Context {}


#[get("/")]
fn template() -> Template {
    let context = Context{};
    Template::render("index", &context)
}


fn main() {
    println!("Hello, world!");

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("static"))
        .mount("/", routes![hello, template])
        .mount("/manager", routes![views::manager::index,
                                   views::manager::issues,
                                   views::manager::statistics,
                                   views::manager::settings])
        .mount("/technician", routes![views::technician::index,
                                      views::technician::issues,
                                      views::technician::statistics,
                                      views::technician::settings])
        .launch();
}
