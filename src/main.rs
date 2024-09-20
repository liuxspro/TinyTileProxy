#[macro_use]
extern crate rocket;

use figment::Figment;
use rocket::http::ContentType;
use rocket::response::content::RawXml;
use rocket::Config;
use rocket::State;
use rocket::{http::Status, response::status};
use rust_embed::Embed;

mod libs;

use libs::geocloud::get_geocloud_tile;
use libs::utils::{create_default_config_file, get_local_config_data, get_tk_from_local_config};

#[derive(FromForm)]
struct GeoCloudQuery {
    layer: String,
}

struct Token {
    tk: String,
}

#[get("/getTile/geocloud/<z>/<x>/<y>?<query..>")]
async fn get_geocloud(
    z: u32,
    x: u32,
    y: u32,
    query: GeoCloudQuery,
    token: &State<Token>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    match get_geocloud_tile(z, x, y, query.layer, token.tk.clone()).await {
        Ok(body) => Ok((ContentType::PNG, body)),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error is: {}", e),
        )),
    }
}

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
    create_default_config_file().unwrap();
    let local_config = get_local_config_data();
    let figment = Figment::from(rocket::Config::default()).merge(local_config.nested());

    let config: Config = figment.extract().expect("Failed to extract config");

    // 获取 port 值
    let port = config.port;
    println!("使用: 将 http://127.0.0.1:{port}/WMTS/geocloud 添加为 QGIS WMTS 连接\n");

    // // 获取 tk 值
    let tk = get_tk_from_local_config();
    rocket::custom(figment)
        .manage(Token { tk })
        .mount("/", routes![get_geocloud, get_geocloud_wmts])
}
