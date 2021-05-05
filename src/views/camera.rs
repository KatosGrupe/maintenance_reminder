use crate::MaintenanceDb;
use crate::login::Technician;
use crate::login::User;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct UserContext {
    user: User
}

#[get("/cameras", rank = 2)]
pub fn cameras_redirect_unauthorized() -> Redirect {
    Redirect::to(uri!(crate::views::login::login: "Neautorizuotas vartotojas"))
}

#[derive(Debug, Queryable, Serialize)]
struct Camera {
    id: i32,
    name: String
}

#[derive(Serialize)]
struct CamerasContext {
    user: User,
    cameras: Vec<Camera>,
}

fn load_cameras(db: &diesel::PgConnection) -> Vec<Camera> {
    use crate::schema::cameras::dsl::*;
    use diesel::prelude::*;
    cameras.load::<Camera>(db).expect("Failed to load cameras")
}

#[get("/cameras", rank = 1)]
pub fn cameras(technician: Technician, db: MaintenanceDb) -> Template {
    let context = CamerasContext {
        cameras: load_cameras(&db),
        user: technician.user
    };
    Template::render("cameras", &context)
}
