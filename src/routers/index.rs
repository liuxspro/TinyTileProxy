use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::Redirect;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/docs/index.html"))
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to(uri!("/docs/favicon.ico"))
}

#[derive(Debug)]
pub struct HostHeader(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(host) = request.headers().get_one("Host") {
            Outcome::Success(HostHeader(host.to_string()))
        } else {
            Outcome::Error((Status::BadRequest, ()))
        }
    }
}

#[get("/host")]
pub fn get_host(host: HostHeader) -> String {
    format!("Host: {}", host.0)
}
