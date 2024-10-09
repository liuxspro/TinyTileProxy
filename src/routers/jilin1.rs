use rocket::http::ContentType;
use rocket::State;
use rocket::{http::Status, response::status};

use crate::libs::jilin1::get_jl_tile;
use crate::libs::utils::{is_webp, webp_to_png, ServerConfig};

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
    if tk == "" {
        // eprintln!("Error: jilin1 tk not set");
        return Err(status::Custom(
            Status::InternalServerError,
            format!("Error: jilin1 tk not set"),
        ));
    }
    // 计算翻转后的Y
    let reversed_y: u32 = (1u32 << z) - 1 - y;
    match get_jl_tile(z, x, reversed_y, query.mk, tk.to_string()).await {
        Ok(body) => {
            if is_webp(&body) {
                //当你对 Vec<u8> 取引用时，你得到的是一个 &[u8]
                let png_data = webp_to_png(body).unwrap();
                Ok((ContentType::PNG, png_data))
            } else {
                // eprintln!("{}/{}/{} : 原始图像不是webp", z, x, reversed_y);
                Ok((ContentType::PNG, body))
            }
        }
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Error is: {}", e),
        )),
    }
}
