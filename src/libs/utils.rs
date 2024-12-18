use anyhow::{anyhow, Result as AnyhowResult};

use image::ImageFormat;
use std::{io::Write, path::PathBuf};

use std::fs::File;
use std::path::Path;
use std::{collections::HashMap, io::Cursor};

pub struct ZXY {
    pub z: String,
    pub x: u32,
    pub y: u32,
}

pub fn is_png(data: &[u8]) -> bool {
    let png_magic_number: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    // 检查数据长度是否至少为 8 字节
    if data.len() < 8 {
        return false;
    }
    // 比较前 8 个字节与 PNG 魔数
    &data[..8] == png_magic_number
}

pub fn is_webp(data: &[u8]) -> bool {
    if data.len() < 12 {
        return false;
    }

    // 检查文件头是否为 "RIFF"
    if &data[0..4] != b"RIFF" {
        return false;
    }

    // 检查文件类型是否为 "WEBP"
    if &data[8..12] != b"WEBP" {
        return false;
    }

    true
}

pub fn webp_to_png(webp_data: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 使用 Cursor 将 Vec<u8> 转换为可读的流
    let reader = Cursor::new(webp_data);

    // 解码 WebP 图片
    let img = image::load(reader, ImageFormat::WebP)?;

    // 创建一个 Vec<u8> 来存储 PNG 数据
    let mut png_data = Vec::new();

    // 将图片保存为 PNG 格式
    img.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)?;

    Ok(png_data)
}

pub fn create_cache_dir() {
    // 获取当前工作目录
    let cache_dir = get_cache_dir();
    if !cache_dir.exists() {
        // 创建 Cache 文件夹
        println!("Create Cache dir");
        std::fs::create_dir(&cache_dir).expect("无法创建 Cache 文件夹");
    }
}

pub fn get_cache_dir() -> PathBuf {
    // 获取当前工作目录
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    current_dir.join("Cache")
}

pub fn read_file(file_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file_path)
}

pub fn get_map_names() -> HashMap<&'static str, &'static str> {
    let mut map_names = HashMap::new();
    map_names.insert("qg250w_20210416_ZAZSeOGX", "全国1比250万地质图");
    map_names.insert("qg150w_20210416_BIwqE0wU", "全国1比150万地质图");
    map_names.insert("全国100万地质图_20210330_rpam5kdJ", "全国1比100万地质图");
    map_names.insert("qg50w_20210416_F7qGy9A7", "全国1比50万地质图");
    map_names.insert("qg20_20210401_FCnDDRJd", "全国1比20万地质图");
    map_names.insert(
        "gisdatamanageCZL:500wdxszyt2616300_20220905_BhR4tgbF",
        "中国地下水资源图",
    );
    map_names.insert(
        "gisdatamanageCZL:zgswdzt16175436_20220905_BbcQipWD",
        "中国水文地质图",
    );
    map_names.insert(
        "73ad26c4aa6957eef051ecc5a15308b4",
        "2023年度全国高质量一张图",
    );
    map_names
}

pub fn save_png(tile_path: PathBuf, buffer: &[u8]) -> AnyhowResult<bool> {
    if is_png(&buffer) {
        let mut tile_file = File::create(&tile_path)?;
        tile_file.write_all(&buffer)?;
        Ok(true)
    } else {
        Err(anyhow!("Filed to save: Not A PNG File"))
    }
}
