use crate::login::Technician;
use chrono::Duration;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[derive(Serialize)]
struct IssuesRegisterContext {
    current_datetime: String,
}

#[get("/cameras")]
pub fn cameras() -> Template {
    let context = Context {};
    Template::render("technician/cameras", &context)
}

#[get("/cameras/register")]
pub fn cameras_register() -> Template {
    let context = Context {};
    Template::render("technician/cameras.register", &context)
}

#[derive(Serialize)]
struct CamerasEditContext {
    name: String,
}

#[get("/cameras/<id>")]
pub fn cameras_edit(id: i32) -> Template {
    let context = match id {
        4 => CamerasEditContext {
            name: "Aleksoto ekrano kamera".to_string(),
        },
        5 => CamerasEditContext {
            name: "Kauno ofiso kamera".to_string(),
        },
        6 => CamerasEditContext {
            name: "Ateities ekrano kamera".to_string(),
        },
        _ => CamerasEditContext {
            name: "".to_string(),
        },
    };

    Template::render("technician/cameras.edit", &context)
}

#[get("/cameras/issues/register")]
pub fn cameras_issues_register() -> Template {
    let context = IssuesRegisterContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    };
    Template::render("technician/cameras.issues.register", &context)
}

#[get("/cameras/issues/<id>")]
pub fn cameras_issues_edit(id: i32) -> Template {
    let context = IssuesRegisterContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    };
    match id {
        1 => Template::render("technician/cameras.issues.history", &context),
        2 => Template::render("technician/cameras.issues.solve", &context),
        _ => Template::render("technician/cameras.issues.react", &context),
    }
}

#[get("/")]
pub fn index(_technician: Technician) -> Template {
    let context = Context {};
    Template::render("technician/index", &context)
}

#[get("/issues")]
pub fn issues() -> Template {
    let context = Context {};
    Template::render("technician/issues", &context)
}

use chrono::prelude::*;

#[get("/issues/register")]
pub fn issues_register() -> Template {
    let context = IssuesRegisterContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    };
    Template::render("technician/issues.register", &context)
}

#[derive(Serialize)]
struct IssuesSolveContext {
    current_datetime: String,
}

#[get("/issues/<id>")]
pub fn issues_solve(id: i32) -> Template {
    let context = IssuesSolveContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    };
    match id {
        1 => Template::render("technician/issues.history", &context),
        2 => Template::render("technician/issues.solve", &context),
        _ => Template::render("technician/issues.take", &context),
    }
}

#[get("/inventory")]
pub fn inventory() -> Template {
    let context = Context {};
    Template::render("technician/inventory", &context)
}

#[get("/inventory/register")]
pub fn inventory_register() -> Template {
    let context = Context {};
    Template::render("technician/inventory.register", &context)
}

#[derive(Serialize)]
struct InventoryInfoContext {
    name: String,
    locations: Vec<Location>,
}

#[derive(Serialize)]
struct Location {
    id: i32,
    location: String,
    lc_time: String,
}

#[get("/inventory/<id>")]
pub fn inventory_info(id: i32) -> Template {
    let context = match id {
        13 => InventoryInfoContext {
            name: "AIO kompas HP Intel i5".to_string(),
            locations: vec![],
        },
        22 => InventoryInfoContext {
            name: "RAM DDR3 2200MHz".to_string(),
            locations: vec![
                Location {
                    id: 1,
                    location: "Kauno ofisas".to_string(),
                    lc_time: "2019-11-04 13:23".to_string(),
                },
                Location {
                    id: 2,
                    location: "Kauno ofisas".to_string(),
                    lc_time: "2019-11-04 13:24".to_string(),
                },
            ],
        },
        27 => InventoryInfoContext {
            name: "LED plokštė S/N: xxxx".to_string(),
            locations: vec![
                Location {
                    id: 1,
                    location: "Kauno ofisas".to_string(),
                    lc_time: "2018-11-04 13:23".to_string(),
                },
                Location {
                    id: 2,
                    location: "Kauno ofisas".to_string(),
                    lc_time: "2018-11-04 14:24".to_string(),
                },
                Location {
                    id: 3,
                    location: "Vilniaus ofisas".to_string(),
                    lc_time: "2018-11-04 13:23".to_string(),
                },
                Location {
                    id: 4,
                    location: "Vilniaus ofisas".to_string(),
                    lc_time: "2018-11-04 14:24".to_string(),
                },
            ],
        },
        _ => InventoryInfoContext {
            name: "".to_string(),
            locations: vec![],
        },
    };
    Template::render("technician/inventory.info", &context)
}

#[get("/inventory/<_part_id>/<_id>")]
pub fn inventory_single_info(_part_id: i32,_id: i32) -> Template {
    let context = Context{};
    Template::render("technician/inventory.single.info", &context)
}

#[derive(Serialize)]
struct StatisticsContext {
    from_date: String,
    to_date: String

}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = StatisticsContext {
        from_date: "2020-01-01".to_string(),
        to_date: "2020-12-31".to_string()
    };
    Template::render("technician/statistics", &context)
}

#[get("/statistics/<_id>")]
pub fn statistics_info(_id: i32) -> Template {
    let context = Context {};
    Template::render("technician/statistics.info", &context)
}

#[get("/screens")]
pub fn screens() -> Template {
    let context = Context {};
    Template::render("technician/screens", &context)
}

#[derive(Serialize)]
struct ScreenPreventiveContext {
    current_datetime: String,
    next_datetime: String
}

#[get("/screens/<_screen_id>/preventitive/<_preventitive_id>")]
pub fn screens_preventitives(_screen_id: i32, _preventitive_id: i32) -> Template {
    let context = ScreenPreventiveContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        next_datetime: (Local::now() + Duration::days(180)).format("%Y-%m-%dT%H:%M:%S").to_string()

    };
    Template::render("technician/screens.preventitives", &context)
}

#[get("/settings")]
pub fn settings() -> Template {
    let context = Context {};
    Template::render("technician/settings", &context)
}

#[get("/camera_issues/register")]
pub fn settings_edit() -> Template {
    let context = IssuesSolveContext {
        current_datetime: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    };
    Template::render("technician/settings.edit", &context)
}

#[get("/tasks")]
pub fn tasks() -> Template {
    let context = Context {

    };
    Template::render("technician/tasks", &context)
}

#[get("/tasks/register")]
pub fn tasks_register() -> Template {
    let context = IssuesSolveContext {
        current_datetime: Local::now().format("%Y-%m-%d").to_string(),
    };
    Template::render("technician/tasks.register", &context)
}

#[get("/tasks/<_id>")]
pub fn tasks_info(_id: i32) -> Template {
    let context = IssuesSolveContext {
        current_datetime: Local::now().format("%Y-%m-%d").to_string(),
    };
    Template::render("technician/tasks.info", &context)
}
