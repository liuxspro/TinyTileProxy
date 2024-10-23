use minijinja::{context, Environment};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::content::RawXml;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

#[derive(Debug)]
pub struct HostFromHeader(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostFromHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(host) = request.headers().get_one("Host") {
            Outcome::Success(HostFromHeader(host.to_string()))
        } else {
            Outcome::Error((Status::BadRequest, ()))
        }
    }
}

#[get("/WMTS/geocloud")]
pub fn get_geocloud_wmts(host: HostFromHeader) -> RawXml<String> {
    let address = format!("http://{}", host.0);

    let wmts_xml = Asset::get("templates/geocloud.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    let mut env = Environment::new();
    env.add_template("geocloud.xml", &file_content).unwrap();
    let template = env.get_template("geocloud.xml").unwrap();
    let rendered = template.render(context! {base_url=> &address}).unwrap();
    RawXml(rendered)
}

#[get("/WMTS/jl1")]
pub fn get_jl1_wmts(host: HostFromHeader) -> RawXml<String> {
    let address = format!("http://{}", host.0);

    let wmts_xml = Asset::get("templates/jl1.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    let mut env = Environment::new();
    env.add_template("jl1.xml", &file_content).unwrap();
    let template = env.get_template("jl1.xml").unwrap();
    let rendered = template.render(context! {base_url=> &address}).unwrap();
    RawXml(rendered)
}

#[get("/WMTS/XYZ")]
pub fn get_xyz_wmts() -> RawXml<String> {
    let wmts_xml = Asset::get("wmts/xyz.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    RawXml(file_content)
}

pub fn routers() -> Vec<rocket::Route> {
    routes![get_geocloud_wmts, get_jl1_wmts, get_xyz_wmts,]
}
