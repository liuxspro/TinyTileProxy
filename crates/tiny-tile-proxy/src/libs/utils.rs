use anyhow::{anyhow, Result as AnyhowResult};

use std::{io::Write, path::PathBuf};

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use filetype::is_png;

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
    if is_png(buffer) {
        let mut tile_file = File::create(&tile_path)?;
        tile_file.write_all(buffer)?;
        Ok(true)
    } else {
        Err(anyhow!("Filed to save: Not A PNG File"))
    }
}
