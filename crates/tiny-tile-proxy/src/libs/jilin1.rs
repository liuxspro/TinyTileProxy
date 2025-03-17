use anyhow::{anyhow, Result as AnyhowResult};
use std::collections::HashMap;
use std::fs::create_dir_all;
use webp_to_png::webp_to_png;

use super::utils::{get_cache_dir, read_file, save_png};

use filetype::is_webp;
use jilin1::get_tile;

pub async fn get_tile_from_cache(
    z: u32,
    x: u32,
    y: u32,
    mk: String,
    tk: String,
) -> AnyhowResult<Vec<u8>> {
    let map_names = get_map_names();
    let cache_dir = get_cache_dir();
    let map_dir = cache_dir.join(format!(
        "吉林1号/{}",
        map_names.get(mk.as_str()).unwrap_or(&mk.as_str())
    ));
    let tile_dir = map_dir.join(format!("{}/{}/", z, x));
    let tile_path = tile_dir.join(format!("{}.png", y));
    if tile_path.exists() {
        let png_data = read_file(&tile_path)?;
        Ok(png_data)
    } else {
        create_dir_all(&tile_dir).expect("Filed to create Tile Dir");
        match get_tile(z, x, y, mk, tk).await {
            Ok(body) => {
                // 缓存瓦片，将 webp 转为 png 保存
                // 存在一些瓦片实际上是 png 格式的(透明)，这里做一下检查
                if is_webp(&body) {
                    let png_data = webp_to_png(body).unwrap();
                    save_png(tile_path, &png_data)?;
                    Ok(png_data)
                } else {
                    save_png(tile_path, &body)?;
                    Ok(body)
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub async fn get_jlearth_tile(z: u32, x: u32, y: u32, tk: String) -> AnyhowResult<Vec<u8>> {
    const AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    let url = format!(
        "https://tile.charmingglobe.com/tile/china2023_5_shield/wmts/{}/{}/{}?v=v1&token={}",
        z, x, y, tk
    );
    // 获取瓦片内容
    let client = reqwest::Client::builder()
        .user_agent(AGENT)
        .gzip(true)
        .build()?;
    // 发送 GET 请求
    let response = client.get(url).send().await?;
    let content_type = response
        .headers()
        .get("Content-Type")
        .ok_or(anyhow!("Missing Content-Type header"))?;

    if content_type == "image/webp" {
        let body = response.bytes().await?;
        Ok(body.to_vec())
    } else {
        Err(anyhow!(
            "Expected Content-Type 'image/webp', but got {:?}",
            content_type
        ))
    }
}

pub async fn get_earthtile_from_cache(z: u32, x: u32, y: u32, tk: String) -> AnyhowResult<Vec<u8>> {
    let cache_dir = get_cache_dir();
    let map_dir = cache_dir.join("吉林1号/2023年度全国高质量一张图 - 共生地球");
    let tile_dir = map_dir.join(format!("{}/{}/", z, x));
    let tile_path = tile_dir.join(format!("{}.png", y));
    if tile_path.exists() {
        let png_data = read_file(&tile_path)?;
        Ok(png_data)
    } else {
        create_dir_all(&tile_dir).expect("Filed to create Tile Dir");
        match get_jlearth_tile(z, x, y, tk).await {
            Ok(body) => {
                // 缓存瓦片，将 webp 转为 png 保存
                // 存在一些瓦片实际上是 png 格式的(透明)，这里做一下检查
                if is_webp(&body) {
                    let png_data = webp_to_png(body).unwrap();
                    save_png(tile_path, &png_data)?;
                    Ok(png_data)
                } else {
                    save_png(tile_path, &body)?;
                    Ok(body)
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub fn get_map_names() -> HashMap<&'static str, &'static str> {
    let mut map_names = HashMap::new();

    map_names.insert(
        "73ad26c4aa6957eef051ecc5a15308b4",
        "2023年度全国高质量一张图",
    );
    map_names
}
