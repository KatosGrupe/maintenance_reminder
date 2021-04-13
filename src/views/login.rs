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

pub fn check_login(user_data: &LoginForm, db: &diesel::PgConnection) -> Option<i32>
{
    use crate::schema::users::dsl::*;
    use crate::user_schema::crypt;
    use diesel::prelude::*;
    let result = users.select(id)
                      .filter(email.eq(user_data.email.clone())
                              .and(password.eq(crypt(user_data.password.clone(), password))))
                      .load::<i32>(db)
                      .unwrap();

    match result.first() {
        Some(val) => Some(val.clone()),
        None => None
    }
}

pub fn get_permissions(id: i32, db: &diesel::PgConnection) -> Vec<i32> {
    use diesel::prelude::*;
    use crate::schema::user_permissions::dsl::*;

    user_permissions.select(permission)
                    .filter(user_id.eq(id))
                    .load::<i32>(db)
                    .unwrap()
}

#[post("/login", data="<user_data>")]
pub fn login_action(user_data: Form<LoginForm>, db: MaintenanceDb, mut cookies: Cookies)
                    -> Redirect {
    return match check_login(&user_data, &db){
        Some(user_id) => {
            cookies.add_private(
                Cookie::new("user_id", user_id.to_string()));

            for perm in get_permissions(user_id, &db) {
                match perm {
                    1 => cookies.add_private(Cookie::new("is_technician", user_id.to_string())),
                    2 => cookies.add_private(Cookie::new("is_manager", user_id.to_string())),
                    3 => cookies.add_private(Cookie::new("is_analyst", user_id.to_string())),
                    x => panic!("Unknown permission class: {}", x)
                }
            }

            // cookies.get_private("user_id", user_data.email);
            Redirect::to(uri!(crate::views::technician::index))
        }
        None => {
            Redirect::to(
                uri!(
                    login: "Neteisingas vartotojo vardas arba slaptažodis"))
        }
    }
}


#[get("/logout")]
pub fn logout_action(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("is_technician"));
    cookies.remove_private(Cookie::named("is_manager"));
    cookies.remove_private(Cookie::named("is_analyst"));
    Redirect::to(
        uri!(
            login: "Sėkmingai atsijungta"
        )
    )
}
