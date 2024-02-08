#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::rocket;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let mut response = client.get(uri!(super::index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }
}
