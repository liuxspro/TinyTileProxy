use anyhow::{anyhow, Result as AnyhowResult};

use std::{io::Write, path::PathBuf};

use std::fs::File;
use std::path::Path;

use filetype::is_png;
// use geocloud::get_map_names;

// use super::geocloud::get_map_names as get_geocloud_map_names;

pub struct ZXY {
    pub z: String,
    pub x: u32,
    pub y: u32,
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

pub fn save_png(tile_path: PathBuf, buffer: &[u8]) -> AnyhowResult<bool> {
    if is_png(buffer) {
        let mut tile_file = File::create(&tile_path)?;
        tile_file.write_all(buffer)?;
        Ok(true)
    } else {
        Err(anyhow!("Filed to save: Not A PNG File"))
    }
}
