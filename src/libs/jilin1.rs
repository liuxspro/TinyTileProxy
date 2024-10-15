use crate::libs::utils::{get_map_names, is_webp, read_file, webp_to_png};
use std::fs::{create_dir_all, File};
use std::io::Write;

type GetTileResult<T> = Result<T, reqwest::Error>;

/// 请求吉林1号瓦片
///
/// ## 参数
///
/// - `z` - z值
/// - `x` - x值
/// - `y` - y值 (这里是标准的XYZ的Y, 会转换为 -Y 进行请求)
/// - `mk` - 地图 mk
/// - `tk` - Token
/// ## Returns
///
/// 返回瓦片二进制数据 Result<Vec<u8>, reqwest::Error>
pub async fn get_jl_tile(z: u32, x: u32, y: u32, mk: String, tk: String) -> GetTileResult<Vec<u8>> {
    const AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    // 计算翻转后的Y
    let reversed_y: u32 = (1u32 << z) - 1 - y;
    let url = format!(
        "https://api.jl1mall.com/getMap/{}/{}/{}?mk={}&tk={}",
        z, x, reversed_y, mk, tk
    );
    // 获取瓦片内容
    // 创建一个客户端并启用 Gzip 解压缩
    let client = reqwest::Client::builder()
        .user_agent(AGENT)
        .gzip(true)
        .build()?;
    // 发送 GET 请求
    let response = client.get(url).send().await?;
    let body = response.bytes().await?;
    Ok(body.to_vec())
}

pub async fn get_tile_from_cache(
    z: u32,
    x: u32,
    y: u32,
    mk: String,
    tk: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let map_names = get_map_names();
    let cache_dir = std::env::current_dir()?.join("Cache");
    let map_dir = cache_dir.join(format!(
        "吉林1号/{}",
        map_names.get(mk.as_str()).unwrap_or(&mk.as_str())
    ));
    let tile_dir = map_dir.join(format!("{}/{}/", z, x));
    let tile_path = tile_dir.join(format!("{}.png", y));
    if tile_path.exists() {
        // println!("瓦片已经缓存");
        let png_data = read_file(&tile_path)?;
        Ok(png_data)
    } else {
        // println!("瓦片未缓存");
        create_dir_all(&tile_dir).expect("Filed to create Tile Dir");
        match get_jl_tile(z, x, y, mk, tk).await {
            Ok(body) => {
                // 缓存瓦片，将 webp 转为 png 保存
                let mut tile_file = File::create(&tile_path)?;
                // 存在一些瓦片实际上是 png 格式的(透明)，这里做一下检查
                if is_webp(&body) {
                    let png_data = webp_to_png(body).unwrap();
                    tile_file.write_all(&png_data)?;
                    Ok(png_data)
                } else {
                    tile_file.write_all(&body)?;
                    Ok(body)
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
