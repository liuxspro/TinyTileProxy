#[macro_use]
extern crate rocket;

use rocket::response::content::RawXml;
use rocket::{http::Status, response::status};
use rust_embed::Embed;

mod libs;
use libs::geocloud::get_geocloud_tile;
use rocket::http::ContentType;

#[derive(FromForm)]
struct GeoCloudQuery {
    layer: String,
}

#[get("/getTile/geocloud/<z>/<x>/<y>?<query..>")]
async fn get_geocloud(
    z: u32,
    x: u32,
    y: u32,
    query: GeoCloudQuery,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    match get_geocloud_tile(z, x, y, query.layer).await {
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
    println!("Tiny Tile Proxy");
    println!("");
    println!("使用: 将 http://127.0.0.1:8000/WMTS/geocloud 添加为 QGIS WMTS 连接\n");
    rocket::build().mount("/", routes![get_geocloud, get_geocloud_wmts])
}
