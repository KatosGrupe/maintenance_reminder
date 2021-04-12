use rocket::Outcome::Forward;
use rocket::Outcome::Success;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;

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
