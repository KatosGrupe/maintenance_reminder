use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {}

#[get("/")]
pub fn index() -> Template {
    let context = Context{};
    Template::render("technician/index", &context)
}

#[get("/issues")]
pub fn issues() -> Template {
    let context = Context{};
    Template::render("technician/issues", &context)
}

#[get("/issues/register")]
pub fn issues_register() -> Template {
    let context = Context{};
    Template::render("technician/issues.register", &context)
}

#[get("/issues/<id>")]
pub fn issues_solve(id: i32) -> Template {
    let context = Context{
    };
    match id {
        1 => Template::render("technician/issues.history", &context),
        2 => Template::render("technician/issues.solve", &context),
        _ => Template::render("technician/issues.take", &context)
    }
}


#[get("/inventory")]
pub fn inventory() -> Template {
    let context = Context{};
    Template::render("technician/inventory", &context)
}

#[get("/inventory/register")]
pub fn inventory_register() -> Template {
    let context = Context{};
    Template::render("technician/inventory.register", &context)
}

#[derive(Serialize)]
struct InventoryInfoContext {
    name: String,
    locations: Vec<Location>
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
            locations: vec![
            ]
        },
        22 => InventoryInfoContext {
            name: "RAM DDR3 2200MHz".to_string(),
            locations: vec![
                Location {id:1,
                          location:"Kauno ofisas".to_string(),
                          lc_time:"2019-11-04 13:23".to_string()},

                Location {id:2,
                          location:"Kauno ofisas".to_string(),
                          lc_time:"2019-11-04 13:24".to_string()},
            ]
        },
        27 => InventoryInfoContext {
            name: "LED plokštė S/N: xxxx".to_string(),
            locations: vec![
                Location {id:1,
                          location:"Kauno ofisas".to_string(),
                          lc_time:"2018-11-04 13:23".to_string()},

                Location {id:2,
                          location:"Kauno ofisas".to_string(),
                          lc_time:"2018-11-04 14:24".to_string()},

                Location {id:3,
                          location:"Vilniaus ofisas".to_string(),
                          lc_time:"2018-11-04 13:23".to_string()},

                Location {id:4,
                          location:"Vilniaus ofisas".to_string(),
                          lc_time:"2018-11-04 14:24".to_string()},
            ]
        },
        _ => InventoryInfoContext{ name: "".to_string(), locations: vec![]}
    };
    Template::render("technician/inventory.info", &context)
}

#[get("/statistics")]
pub fn statistics() -> Template {
    let context = Context{};
    Template::render("technician/statistics", &context)
}

#[get("/statistics/<_id>")]
pub fn statistics_info(_id: i32) -> Template {
    let context = Context{};
    Template::render("technician/statistics.info", &context)
}

#[get("/settings")]
pub fn settings() -> Template {
    let context = Context{};
    Template::render("technician/settings", &context)
}
