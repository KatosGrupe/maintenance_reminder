use crate::MaintenanceDb;
use crate::login::Technician;
use crate::login::User;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct UserContext {
    user: User
}

#[get("/cameras", rank = 1)]
pub fn cameras(technician: Technician, db: MaintenanceDb) -> Template {
    let context = CamerasContext {
        cameras: load_cameras(&db),
        user: technician.user
    };
    Template::render("cameras", &context)
}

#[get("/cameras/<_path..>", rank = 2)]
pub fn cameras_redirect_unauthorized(_path: PathBuf) -> Redirect {
    Redirect::to(uri!(crate::views::login::login: "Neautorizuotas vartotojas"))
}

#[derive(Debug, Queryable, Serialize)]
struct Camera {
    id: i32,
    name: String
}

#[derive(Serialize)]
struct CamerasContext {
    cameras: Vec<Camera>,
    user: User,
}

fn load_cameras(db: &diesel::PgConnection) -> Vec<Camera> {
    use crate::schema::cameras::dsl::*;
    use diesel::prelude::*;
    cameras.load::<Camera>(db).expect("Failed to load cameras")
}


#[derive(Serialize)]
struct CameraEditContext {
    name: String,
    user: User,
}

fn get_camera(id: i32, db: &diesel::PgConnection) -> Option<Camera> {
    // use crate::schema::cameras::dsl::*;
    // use crate::schema::cameras::dsl::filter;
    use crate::schema::cameras::dsl::cameras;
    use diesel::prelude::*;

    cameras.filter(crate::schema::cameras::dsl::id.eq(id))
           .limit(1)
           .load::<Camera>(db)
           .expect("Failed to load cameras").into_iter().nth(0)
}

fn create_camera(camera: Camera, db:&diesel::PgConnection) {
    use crate::schema::cameras::dsl::*;
    use diesel::prelude::*;
    use diesel::insert_into;

    insert_into(cameras).values(name.eq(camera.name))
                        .execute(db)
                        .expect("Failed to create camera");
}

fn edit_camera(camera: Camera, db: &diesel::PgConnection) {
    use crate::schema::cameras::dsl::*;
    use diesel::prelude::*;
    diesel::update(cameras).filter(id.eq(camera.id))
                           .set(name.eq(camera.name))
                           .execute(db)
                           .expect("Failed to update cameras");
}

#[get("/cameras/<id>", rank = 1)]
pub fn camera_edit(id: i32, technician: Technician, db: MaintenanceDb) -> Template {
    let camera = get_camera(id, &db).unwrap_or(Camera {id: 0, name: "".to_string()});
    let context = CameraEditContext {
        name: camera.name,
        user: technician.user
    };

    Template::render("camera_edit", &context)
}

#[derive(FromForm)]
pub struct CameraForm {
    name: String
}

use rocket::request::Form;

#[post("/cameras/<id>", data = "<form>")]
pub fn cameras_edit_action(id: i32, form: Form<CameraForm>,
                           _technician: Technician, db: MaintenanceDb)
                           -> Redirect {
    //Check for camera by id
    let camera = get_camera(id, &db);
    println!("{:#?}", camera);

    match camera {
        Some(_) => {
            edit_camera(Camera {
                id,
                name: form.into_inner().name
            }, &db);
        },
        None => {
            //Create
            create_camera(Camera {
                id: 0, name: form.into_inner().name
            }, &db);

        }
    }

    Redirect::to(
        uri!(cameras)
    )
}
