use crate::MaintenanceDb;
use rocket::Outcome::Forward;
use rocket::Outcome::Success;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    is_analyst: bool,
    is_manager: bool,
    is_technician: bool
}

impl<'a, 'r> FromRequest<'a, 'r> for User
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, Self::Error> {
        let result = request.cookies().get_private("user_id");
        let db = request.guard::<MaintenanceDb>().unwrap();

        match result {
            Some(user_id_cookie) => {
                let permissions = crate::views::login::get_permissions(user_id_cookie.value().parse().unwrap(), &db);
                Success(User {
                    is_analyst: permissions.contains(&3),
                    is_technician: permissions.contains(&1),
                    is_manager: permissions.contains(&2)
                })
            }
            None => Forward(())
        }
    }
}

#[derive(Debug)]
pub struct Technician {
    pub user: User
}

impl<'a, 'r> FromRequest<'a, 'r> for Technician
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Technician, Self::Error> {
        match request.guard::<User>(){
            Success(user) if user.is_technician =>
                Success(Technician {user}),
            _ => {
                Forward(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Manager {
    pub user: User
}

impl<'a, 'r> FromRequest<'a, 'r> for Manager
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Manager, Self::Error> {
        match request.guard::<User>(){
            Success(user) if user.is_manager =>
                Success(Manager {user}),
            _ => {
                Forward(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Analyst {
    pub user: User
}

impl<'a, 'r> FromRequest<'a, 'r> for Analyst
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Analyst, Self::Error> {
        match request.guard::<User>() {
            Success(user) if user.is_manager =>
                Success(Analyst {user}),
            _ => {
                Forward(())
            }
        }
    }
}
