use figment::{
    providers::{Format, Toml},
    Figment,
};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn get_tk_from_local_config() -> String {
    let config = Figment::from(get_local_config_data());
    config
        .extract_inner("geocloud.tk")
        .expect("Filed to get geocloud tk")
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

[geocloud]
tk = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJnZW9jbG91ZCIsImV4cCI6MTcyNzI1NTM3NH0.wdDEJ8-n8ylzF8g2ludHRWVX9TTqLO7u_nRbItX-v7M"
"#;

    // 创建文件并写入内容
    let mut file = File::create(config_path)?;
    file.write_all(config_content.as_bytes())?;

    // println!("create default config.toml");
    Ok(())
}
