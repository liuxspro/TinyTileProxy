use rocket::response::Redirect;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/docs/index.html"))
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to(uri!("/docs/favicon.ico"))
}
