use anyhow::{anyhow, Result as AnyhowResult};
use std::fs::{create_dir_all, File};
use std::io::Write;

use super::utils::{get_cache_dir, get_map_names, is_png, is_webp, read_file, webp_to_png};

/// 请求吉林1号瓦片
///
/// ## 参数
///
/// - `z` - z值
/// - `x` - x值
/// - `y` - y值
/// - `mk` - 地图 mk
/// - `tk` - Token
/// ## Returns
///
/// 返回瓦片二进制数据 Result<Vec<u8>, reqwest::Error>
pub async fn get_jl_tile(z: u32, x: u32, y: u32, mk: String, tk: String) -> AnyhowResult<Vec<u8>> {
    const AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    // 通过添加sch=wmts可返回正常XYZ顺序, 否则使用 `reversed_y: u32 = (1u32 << z) - 1 - y` 计算 -y 值
    let url = format!(
        "https://api.jl1mall.com/getMap/{}/{}/{}?mk={}&tk={}&sch=wmts",
        z, x, y, mk, tk
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
    // TODO tk不正确的时候也返回瓦片（参数有误），应返回错误
    Ok(body.to_vec())
}

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
                let mut tile_file = File::create(&tile_path)?;
                // 存在一些瓦片实际上是 png 格式的(透明)，这里做一下检查
                if is_webp(&body) {
                    let png_data = webp_to_png(body).unwrap();
                    tile_file.write_all(&png_data)?;
                    Ok(png_data)
                } else {
                    if is_png(&body) {
                        tile_file.write_all(&body)?;
                    }
                    Ok(body)
                }
            }
            Err(e) => Err(e),
        }
    }
}
