#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;

pub mod views;
pub mod login;
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

#[derive(Debug)]
pub struct AuroraConfig {
        url: String,
        user: String,
        key: String
}

fn main() {
    pretty_env_logger::init();
    rocket::ignite()
        .attach(Template::fairing())
        .attach(MaintenanceDb::fairing())
        .attach(AdHoc::on_attach("Aurora Rest API Config", |rocket| {
                let table = rocket.config()
                                  .get_extra("aurora_api_config")
                                  .expect("Missing (aurora_api_config) from configuration");
                let config = AuroraConfig{
                        url: table.get("url").expect("Missing (aurora_api_config.url) from configuration").as_str().unwrap().to_string(),
                        user: table.get("user").expect("Missing (aurora_api_config.user) from configuration").as_str().unwrap().to_string(),
                        key: table.get("key").expect("Missing (aurora_api_config.key) from configuration").as_str().unwrap().to_string(),
                };
                println!("{:#?}", config);
                Ok(rocket.manage(config))
        }))
        .mount("/public", StaticFiles::from("static"))
        .mount("/", routes![test,
                            views::camera::camera_edit,
                            views::camera::cameras,
                            views::camera::cameras_edit_action,
                            views::camera::cameras_redirect_unauthorized,
                            views::login::login,
                            views::login::login_action,
                            views::login::logout_action,
                            views::screen::screens,
                            views::screen::screens_redirect_unauthorized,
                            views::technician::index,
        ])
        // .mount("/manager", routes![views::manager::index,
        //                            views::manager::issues,
        //                            views::manager::issues_register,
        //                            views::manager::statistics,
        //                            views::manager::settings])
        // .mount("/super_manager", routes![views::super_manager::index,
        //                                  views::super_manager::statistics])
        // .mount("/technician", routes![
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

