use minijinja::{context, Environment};
use rocket::response::content::RawXml;
use rocket::State;
use rust_embed::Embed;
// use tera::Tera;

use crate::libs::utils::ServerConfig;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

#[get("/WMTS/geocloud")]
pub fn get_geocloud_wmts(config: &State<ServerConfig>) -> RawXml<String> {
    let ip = &config.ip;
    let port = &config.port;
    let address = format!("http://{}:{}", ip, port);

    let wmts_xml = Asset::get("templates/geocloud.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    let mut env = Environment::new();
    env.add_template("geocloud.xml", &file_content).unwrap();
    let template = env.get_template("geocloud.xml").unwrap();
    let rendered = template.render(context! {base_url=> &address}).unwrap();
    RawXml(rendered)
}

#[get("/WMTS/jl1")]
pub fn get_jl1_wmts(config: &State<ServerConfig>) -> RawXml<String> {
    let ip = &config.ip;
    let port = &config.port;
    let address = format!("http://{}:{}", ip, port);

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
