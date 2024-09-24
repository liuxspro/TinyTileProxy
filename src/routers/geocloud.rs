use rocket::http::ContentType;
use rocket::State;
use rocket::{http::Status, response::status};

use crate::libs::geocloud::get_geocloud_tile;
use crate::libs::utils::{ServerConfig, ZXY};

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
    config: &State<ServerConfig>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    let zxy = ZXY {
        z: z.to_string(),
        x,
        y,
    };
    match get_geocloud_tile(
        zxy,
        query.layer,
        config.tokens.geocloud.clone(),
        query.tilematrixset,
    )
    .await
    {
        Ok(body) => Ok((ContentType::PNG, body)),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error is: {}", e),
        )),
    }
}
