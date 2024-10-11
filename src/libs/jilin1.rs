use reqwest::Error as reqwestError;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub async fn get_jl_tile(
    z: u32,
    x: u32,
    y: u32,
    mk: String,
    tk: String,
) -> Result<Vec<u8>, reqwestError> {
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

fn read_file(file_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file_path)
}

pub async fn get_tile_from_cache(
    z: u32,
    x: u32,
    y: u32,
    mk: String,
    tk: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cache_dir = std::env::current_dir()?.join("Cache");
    let map_dir = cache_dir.join(format!("吉林1号/{}", mk));
    let tile_dir = map_dir.join(format!("{}/{}/", z, x));
    let tile_path = tile_dir.join(format!("{}.png", y));
    if tile_path.exists() {
        println!("瓦片已经缓存");
        let png_data = read_file(&tile_path)?;
        Ok(png_data)
    } else {
        create_dir_all(&tile_dir).expect("Filed to create Tile Dir");
        println!("瓦片未缓存");
        match get_jl_tile(z, x, y, mk, tk).await {
            Ok(body) => {
                let mut tile_file = File::create(&tile_path)?;
                tile_file.write_all(&body)?;
                Ok(body)
            }
            Err(e) => Err(e.into()),
        }
    }
}
