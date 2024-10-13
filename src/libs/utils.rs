use figment::{
    providers::{Format, Toml},
    Figment,
};
use image::ImageFormat;
use serde::Deserialize;
use std::io::Cursor;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::path::Path;
use std::{fs::File, net::Ipv4Addr};

#[derive(Deserialize)]
pub struct Tokens {
    pub geocloud: String,
    pub jl1: String,
}

pub struct ZXY {
    pub z: String,
    pub x: u32,
    pub y: u32,
}

pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub tokens: Tokens,
}

pub fn get_tk_from_local_config() -> Result<Tokens, figment::Error> {
    let config: Tokens = Figment::new()
        .merge(get_local_config_data().nested())
        .select("tokens")
        .extract()
        .expect("Filed to get tokens");

    Ok(config)
}

pub fn get_local_config_data() -> figment::providers::Data<Toml> {
    Toml::file("config.toml")
}

pub fn create_default_config_file() -> io::Result<()> {
    let config_path = "config.toml";

    // 检查文件是否存在
    if Path::new(config_path).exists() {
        // println!("config.toml already exists.");
        return Ok(());
    }

    // 定义要写入的内容
    let config_content = r#"[default]
address = "127.0.0.1"
port = 8000

[tokens]
geocloud = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJnZW9jbG91ZCIsImV4cCI6MTcyNzg2MDE4Nn0.Nk9RV5m-5uVcMOzUAo7JtZaamrYDxxRf5-hKvQl3agg"
jl1 = ""
"#;

    // 创建文件并写入内容
    let mut file = File::create(config_path)?;
    file.write_all(config_content.as_bytes())?;

    // println!("create default config.toml");
    Ok(())
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

pub fn get_local_ip() -> Option<IpAddr> {
    let ali_dns = "223.5.5.5:53";
    if let Ok(stream) = TcpStream::connect(ali_dns) {
        if let Some(local_addr) = stream.local_addr().ok() {
            return Some(local_addr.ip());
        }
    }
    None
}

pub fn is_unspecified(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4 == Ipv4Addr::UNSPECIFIED,
        IpAddr::V6(_) => false,
    }
}

// pub fn get_tile_from_cache(map_name: &str, layer_name: &str, z: u32, x: u32, y: u32) {}

pub fn create_cache_dir() {
    // 获取当前工作目录
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    let cache_dir = current_dir.join("Cache");
    if !cache_dir.exists() {
        // 创建 Cache 文件夹
        std::fs::create_dir(&cache_dir).expect("无法创建 Cache 文件夹");
        // println!("Cache 文件夹已创建");
    }
}

pub fn read_file(file_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file_path)
}
