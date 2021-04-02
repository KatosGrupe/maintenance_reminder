use rocket_contrib::templates::Template;
use crate::MaintenanceDb;
use rocket::response::Redirect;
use rocket::request::Form;
use serde::Serialize;
use rocket::http::Cookies;
use rocket::http::Cookie;

#[derive(Serialize)]
struct Context {
    message: String
}

#[get("/?<message>", rank = 2)]
pub fn login(message: Option<String>) -> Template {
    let context = Context {
        message: message.unwrap_or("".to_string())
    };
    Template::render("login", &context)
}

#[derive(Debug, FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

//TODO: Add permissions get
pub fn check_login(user_data: &LoginForm, db: &diesel::PgConnection) -> bool
{
    use crate::schema::users::dsl::*;
    use crate::user_schema::crypt;
    use diesel::prelude::*;
    let result: Vec<String> = users.select(email).filter(email.eq(user_data.email.clone())
                              .and(password.eq(crypt(user_data.password.clone(), password))))
        .limit(1).load(db).unwrap();
    if result.len() == 1 {
        return true;
    }

    return false;

}

#[post("/login", data="<user_data>")]
pub fn login_action(user_data: Form<LoginForm>, db: MaintenanceDb, mut cookies: Cookies)
                    -> Redirect {
    if check_login(&user_data, &db) {
        cookies.add_private(
            Cookie::new("user_id", user_data.email.clone()));
        // cookies.get_private("user_id", user_data.email);
        return Redirect::to(uri!(crate::views::technician::index))
    }

    Redirect::to(
        uri!(
            login: "Neteisingas vartotojo vardas arba slaptažodis"))
}


#[get("/logout")]
pub fn logout_action(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to(
        uri!(
            login: "Sėkmingai atsijungta"
        )
    )
}
