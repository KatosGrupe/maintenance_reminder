#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::Outcome::Forward;
use rocket::outcome::IntoOutcome;
use rocket::Outcome::Success;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;

pub mod views;
pub mod schema;
pub mod user_schema;

#[get("/test")]
fn test() -> String {
    format!("The server is alive!")
}

#[derive(Serialize)]
struct Context {}

#[database("maintenance_db")]
pub struct MaintenanceDb(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(MaintenanceDb::fairing())
        .mount("/public", StaticFiles::from("static"))
        .mount("/", routes![test,
                            views::login::login,
                            views::login::login_action,
                            views::login::logout_action,
                            views::technician::index
        ])
        // .mount("/manager", routes![views::manager::index,
        //                            views::manager::issues,
        //                            views::manager::issues_register,
        //                            views::manager::statistics,
        //                            views::manager::settings])
        // .mount("/super_manager", routes![views::super_manager::index,
        //                                  views::super_manager::statistics])
        // .mount("/technician", routes![views::technician::cameras,
        //                               views::technician::cameras_register,
        //                               views::technician::cameras_edit,
        //                               views::technician::cameras_issues_register,
        //                               views::technician::cameras_issues_edit,
        //                               views::technician::index,
        //                               views::technician::inventory,
        //                               views::technician::inventory_register,
        //                               views::technician::inventory_info,
        //                               views::technician::inventory_single_info,
        //                               views::technician::issues,
        //                               views::technician::issues_register,
        //                               views::technician::issues_solve,
        //                               views::technician::screens,
        //                               views::technician::screens_preventitives,
        //                               views::technician::settings,
        //                               views::technician::settings_edit,
        //                               views::technician::statistics,
        //                               views::technician::statistics_info,
        //                               views::technician::tasks,
        //                               views::technician::tasks_register,
        //                               views::technician::tasks_info
        // ])
        // .mount("/email", routes![views::email::index])
        .launch();
}

pub struct User {
}

impl User {
    fn is_technician(&self) -> bool {
        true
    }

    fn is_manager(&self) -> bool {
        true
    }

    fn is_analyst(&self) -> bool {
        true
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let result = request.cookies().get_private("user_id");
        println!("{:?}", result);
        match result {
            Some(_user_id) => Success(User{}),
            None => Forward(())
        }
    }
}

pub struct Technician {}

impl<'a, 'r> FromRequest<'a, 'r> for Technician {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>()?;
        if user.is_technician() {
            return Success(Technician{});
        }

        Forward(())
    }
}

pub struct Manager {}

impl<'a, 'r> FromRequest<'a, 'r> for Manager {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>)
                    -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>()?;
        if user.is_manager() {
            return Success(Manager{});
        }

        Forward(())
    }
}

pub struct Analyst {}

impl<'a, 'r> FromRequest<'a, 'r> for Analyst {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>)
                    -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>()?;
        if user.is_analyst() {
            return Success(Analyst{});
        }

        Forward(())
    }
}
