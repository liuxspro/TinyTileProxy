use rocket::http::ContentType;
use rocket::State;
use rocket::{http::Status, response::status};

use crate::libs::jilin1::{get_earthtile_from_cache, get_tile_from_cache};
use crate::libs::utils::ServerConfig;

#[derive(FromForm)]
pub(crate) struct JiLin1Query {
    mk: String,
}

#[get("/getTile/jl1/<z>/<x>/<y>?<query..>")]
pub async fn get_jl1(
    z: u32,
    x: u32,
    y: u32,
    query: JiLin1Query,
    config: &State<ServerConfig>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    let tk = &config.tokens.jl1;
    if tk.is_empty() {
        return Err(status::Custom(
            Status::InternalServerError,
            "Error: jilin1 tk not set".to_string(),
        ));
    }
    match get_tile_from_cache(z, x, y, query.mk, tk.to_string()).await {
        Ok(body) => Ok((ContentType::PNG, body)),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error is: {}", e),
        )),
    }
}

#[get("/getTile/jl1earth/<z>/<x>/<y>")]
pub async fn get_jl1earth(
    z: u32,
    x: u32,
    y: u32,
    config: &State<ServerConfig>,
) -> Result<(ContentType, Vec<u8>), status::Custom<String>> {
    let tk = &config.tokens.jl1earth;
    match get_earthtile_from_cache(z, x, y, tk.to_string()).await {
        Ok(body) => Ok((ContentType::PNG, body)),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error: {}", e),
        )),
    }
}
