use rocket::response::content::RawXml;
use rocket::State;
use rust_embed::Embed;
use tera::Tera;

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
    let mut tera = Tera::default();
    tera.add_raw_template("geocloud.xml", &file_content)
        .expect("Filed to add template");
    // 构建模板上下文
    let mut context = tera::Context::new();
    context.insert("base_url", &address);
    let rendered = tera.render("geocloud.xml", &context).unwrap();
    RawXml(rendered)
}

#[get("/WMTS/jl1")]
pub fn get_jl1_wmts(config: &State<ServerConfig>) -> RawXml<String> {
    let ip = &config.ip;
    let port = &config.port;
    let address = format!("http://{}:{}", ip, port);

    let wmts_xml = Asset::get("templates/jl1.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    let mut tera = Tera::default();
    tera.add_raw_template("jl1.xml", &file_content)
        .expect("Filed to add template");
    // 构建模板上下文
    let mut context = tera::Context::new();
    context.insert("base_url", &address);
    let rendered = tera.render("jl1.xml", &context).unwrap();
    RawXml(rendered)
}

#[get("/WMTS/xyz")]
pub fn get_xyz_wmts() -> RawXml<String> {
    let wmts_xml = Asset::get("wmts/xyz.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    RawXml(file_content)
}
