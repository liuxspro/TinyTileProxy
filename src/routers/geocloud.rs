use rocket::http::ContentType;
use rocket::State;
use rocket::{http::Status, response::status};

use crate::libs::geocloud::get_geocloud_tile;
use crate::libs::utils::ServerConfig;

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
    match get_geocloud_tile(
        z,
        x,
        y,
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
