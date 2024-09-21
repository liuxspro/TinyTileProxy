#[macro_use]
extern crate rocket;

use figment::Figment;
use rocket::response::content::RawXml;
use rocket::Config;
use rust_embed::Embed;

mod libs;
mod routers;

use libs::utils::{
    create_default_config_file, get_local_config_data, get_local_ip, get_tk_from_local_config,
    Tokens,
};
use routers::geocloud::get_geocloud;
use routers::index::index;
use routers::jilin1::get_jl1;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

#[get("/WMTS/geocloud")]
fn get_geocloud_wmts() -> RawXml<String> {
    let wmts_xml = Asset::get("wmts/geocloud.xml").unwrap();
    let file_content = String::from_utf8(wmts_xml.data.to_vec()).expect("filed to read");
    RawXml(file_content)
}

#[launch]
fn rocket() -> _ {
    println!("Tiny Tile Proxy\n");
    let ip = get_local_ip().expect("Failed to get local IP address");
    println!("{}", ip);
    create_default_config_file().unwrap();
    let local_config = get_local_config_data();
    let figment = Figment::from(rocket::Config::default()).merge(local_config.nested());

    let config: Config = figment.extract().expect("Failed to extract config");

    // 获取 port 值
    let port = config.port;
    println!("使用: 将 http://127.0.0.1:{port}/WMTS/geocloud 添加为 QGIS WMTS 连接\n");

    // 获取 tk 值
    let tk = get_tk_from_local_config().unwrap();
    if tk.jl1 == "" {
        panic!("Error: jilin1 tk not set");
    }
    rocket::custom(figment)
        .manage(Tokens {
            geocloud: tk.geocloud,
            jl1: tk.jl1,
        })
        .mount(
            "/",
            routes![index, get_geocloud, get_jl1, get_geocloud_wmts],
        )
}
