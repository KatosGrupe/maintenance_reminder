#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use rocket_contrib::databases::diesel;
use serde::Serialize;
use rocket_contrib::serve::StaticFiles;

pub mod views;

#[get("/test")]
fn test() -> String {
    format!("The server is alive!")
}

#[derive(Serialize)]
struct Context {}


#[get("/")]
fn template() -> Template {
    let context = Context{};
    Template::render("index", &context)
}

#[database("maintenance_db")]
pub struct MaintenanceDb(diesel::PgConnection);


fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(MaintenanceDb::fairing())
        .mount("/public", StaticFiles::from("static"))
        .mount("/", routes![test,
                            template,
                            views::login::login,
                            views::login::login_action,
        ])
        .mount("/manager", routes![views::manager::index,
                                   views::manager::issues,
                                   views::manager::issues_register,
                                   views::manager::statistics,
                                   views::manager::settings])
        .mount("/super_manager", routes![views::super_manager::index,
                                         views::super_manager::statistics])
        .mount("/technician", routes![views::technician::cameras,
                                      views::technician::cameras_register,
                                      views::technician::cameras_edit,
                                      views::technician::cameras_issues_register,
                                      views::technician::cameras_issues_edit,
                                      views::technician::index,
                                      views::technician::inventory,
                                      views::technician::inventory_register,
                                      views::technician::inventory_info,
                                      views::technician::inventory_single_info,
                                      views::technician::issues,
                                      views::technician::issues_register,
                                      views::technician::issues_solve,
                                      views::technician::screens,
                                      views::technician::screens_preventitives,
                                      views::technician::settings,
                                      views::technician::settings_edit,
                                      views::technician::statistics,
                                      views::technician::statistics_info,
                                      views::technician::tasks,
                                      views::technician::tasks_register,
                                      views::technician::tasks_info
        ])
        .mount("/email", routes![views::email::index])
        .launch();
}
