use crate::AuroraConfig;
use rocket::State;
use aurora_api_common::models::screen::Screen;
use crate::login::Technician;
use crate::login::User;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::path::PathBuf;
use log::debug;


#[get("/screens/<_path..>", rank = 2)]
pub fn screens_redirect_unauthorized(_path: PathBuf) -> Redirect {
    Redirect::to(uri!(crate::views::login::login: "Neautorizuotas vartotojas"))
}

fn load_screens(aurora_config: &AuroraConfig) -> Vec<Screen> {
    let url = format!("{}/screens/logins", aurora_config.url);
    debug!("Retrieving screen SMB logins ({})", url);

    let mut headers = reqwest::header::HeaderMap::new();
    let mut key = reqwest::header::HeaderValue
        ::from_str(&format!("Basic {}",
                            base64::encode(&format!("{}:{}",
                                                    aurora_config.user,
                                                    aurora_config.key))))
        .expect("Failed to set header");

    key.set_sensitive(true);
    println!("{:?}", key);
    headers.insert(reqwest::header::AUTHORIZATION, key);
    let client = reqwest::blocking::Client::builder().default_headers(headers)
                                           .build().unwrap();
    let result = client
                     .get(&url)
                     .send()
                     .expect(&format!("Failed to GET data ({})", url));
    debug!("aaaa: {:#?}", result);

    serde_json::from_str(&result.text().unwrap()).unwrap()
}

#[derive(Serialize)]
struct ScreenContext {
    screens: Vec<Screen>,
    user: User,
}

#[get("/screens")]
pub fn screens(technician: Technician, aurora_config: State<AuroraConfig>) -> Template {
    let context = ScreenContext {
        screens: load_screens(&*aurora_config),
        user: technician.user
    };
    Template::render("screens", &context)
}
