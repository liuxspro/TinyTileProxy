use figment::{
    providers::{Format, Toml},
    Figment,
};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use std::{fs, path::Path};
use toml::to_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub address: String,
    pub port: u32,
    pub use_https: bool,
    pub password: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: String::from("127.0.0.1"),
            port: 8000,
            use_https: false,
            password: String::from("ttp123456"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Tokens {
    pub geocloud: String,
    pub jl1: String,
    pub jl1earth: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub default: ServerConfig,
    pub tokens: Tokens,
    pub jl1_mk: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut mks = HashMap::new();
        mks.insert(
            "73ad26c4aa6957eef051ecc5a15308b4".to_string(),
            "2023年度全国高质量一张图".to_string(),
        );
        Config {
            default: ServerConfig::default(),
            tokens: Tokens::default(),
            jl1_mk: mks,
        }
    }
}

#[derive(Clone)]
pub struct StateConfig {
    pub tokens: Arc<RwLock<Tokens>>,
    pub use_https: Arc<RwLock<bool>>,
    pub jl1_mk: Arc<RwLock<HashMap<String, String>>>,
}

pub fn get_config() -> Figment {
    Figment::new().merge(Toml::file("config.toml"))
}

/// 创建默认配置文件
pub fn create_default_config_file() {
    let config_path = "config.toml";
    // 检查文件是否存在
    if !Path::new(config_path).exists() {
        println!("create default config file [config.toml]");
        let config = Config::default();
        let toml_str = to_string(&config).expect("Failed to serialize config");
        fs::write(config_path, toml_str).expect("Failed to write config file");
    }
}

/// 从配置文件中获取 Tokens
pub fn get_tk_from_local_config() -> Result<Tokens, figment::Error> {
    let config: Config = get_config().extract()?;
    let tk = config.tokens;

    Ok(tk)
}

pub fn get_jl1_mk_from_local_config() -> Result<HashMap<String, String>, figment::Error> {
    let config: Config = get_config().extract()?;
    let jl1_mk = config.jl1_mk;

    Ok(jl1_mk)
}

/// 保存 Tokens 至配置文件
pub fn save_tokens(tk: &Tokens) -> Result<Tokens, figment::Error> {
    let mut config: Config = get_config().extract()?;
    config.tokens = tk.clone();
    let config_path = "config.toml";
    let toml_str = to_string(&config).expect("Failed to serialize config");
    fs::write(config_path, toml_str).expect("Failed to write config file");
    Ok(tk.clone())
}
