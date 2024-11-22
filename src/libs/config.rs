use figment::{
    providers::{Format, Toml},
    Figment,
};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub default: ServerConfig,
    pub tokens: Tokens,
}

#[derive(Clone)]
pub struct StateConfig {
    pub tokens: Arc<RwLock<Tokens>>,
    pub use_https: Arc<RwLock<bool>>,
}

pub fn get_config() -> Figment {
    let config = Figment::new().merge(Toml::file("config.toml"));
    return config;
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

/// 保存 Tokens 至配置文件
pub fn save_tokens(tk: &Tokens) -> Result<Tokens, figment::Error> {
    let mut config: Config = get_config().extract()?;
    config.tokens = tk.clone();
    let config_path = "config.toml";
    let toml_str = to_string(&config).expect("Failed to serialize config");
    fs::write(config_path, toml_str).expect("Failed to write config file");
    Ok(tk.clone())
}
