use rocket::http::ContentType;
use rocket::State;
use rocket::{http::Status, response::status};
use std::collections::HashMap;

use crate::libs::config::StateConfig;
use crate::libs::geocloud::get_geocloud_tile_cache;
use crate::libs::utils::ZXY;

use filetype::is_png;

#[derive(FromForm)]
pub(crate) struct GeoCloudQuery {
    layer: String,
    tilematrixset: Option<String>,
}

#[get("/getTile/geocloud/<z>/<x>/<y>?<query..>")]
pub async fn get_geocloud(
    z: &str,
    x: u32,
    y: u32,
    query: GeoCloudQuery,
    config: &State<StateConfig>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    let zxy = ZXY {
        z: z.to_string(),
        x,
        y,
    };
    let tokens = config.tokens.read().unwrap().clone();
    match get_geocloud_tile_cache(zxy, query.layer, tokens.geocloud, query.tilematrixset).await {
        Ok(body) => {
            if is_png(&body) {
                Ok((ContentType::PNG, body))
            } else {
                Err(status::Custom(
                    Status::NotFound,
                    format!("{}", String::from_utf8_lossy(&body)),
                ))
            }
        }
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error is: {}", e),
        )),
    }
}

#[get("/geocloud/wms?<params..>")]
pub async fn _geocloud_wms(
    params: HashMap<String, String>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    let base_url =
        "https://igss.cgs.gov.cn:6160/igs/rest/ogc/doc/H50E022002_20201014_QusseidO/WMSServer";
    let client = reqwest::Client::builder().build().unwrap();
    // 发送 GET 请求
    let response = client.get(base_url).query(&params).send().await.unwrap();
    // println!("{} ", response.url().as_str());
    let body = response.bytes().await.unwrap();
    // println!("{:?}", body);
    Ok((ContentType::PNG, body.to_vec()))
    // format!("{:?}", params)
}
