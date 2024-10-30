#[macro_use]
extern crate rocket;

use figment::{providers::Env, Figment};

mod libs;
mod routers;

use libs::utils::{
    create_cache_dir, create_default_config_file, get_local_config_data, get_tk_from_local_config,
    ServerConfig,
};
use routers::docs::{docs, static_file};
use routers::geocloud::get_geocloud;
use routers::index::{favicon, index};
use routers::jilin1::{get_jl1, get_jl1earth};
use routers::wmts;

#[launch]
fn rocket() -> _ {
    create_default_config_file().unwrap();
    create_cache_dir();

    println!("Tiny Tile Proxy\t v{}\n", env!("CARGO_PKG_VERSION"));

    let local_config = get_local_config_data();
    let figment = Figment::from(rocket::Config::default())
        .merge(local_config.nested())
        .merge(Env::prefixed("ROCKET_").global());

    // 获取 tk 值
    let tk = get_tk_from_local_config().unwrap();
    let use_https_str: String = std::env::var("ROCKET_USE_HTTPS").unwrap_or("false".to_string());
    let use_https = use_https_str
        .parse::<bool>()
        .expect("Failed to parse use_https as bool");

    // 检查tk值是否为空
    if tk.jl1.is_empty() {
        eprintln!("吉林一号 tk 值未设置,请在 config.toml 中输入 tk 后重新运行...\n");
    }

    let mut routers = routes![
        index,
        get_geocloud,
        get_jl1,
        get_jl1earth,
        docs,
        static_file,
        favicon,
    ];
    routers.extend(wmts::routers());

    rocket::custom(figment)
        .manage(ServerConfig {
            tokens: tk,
            use_https,
        })
        .mount("/", routers)
}
