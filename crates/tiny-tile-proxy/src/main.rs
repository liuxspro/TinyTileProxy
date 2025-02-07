#[macro_use]
extern crate rocket;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use std::sync::{Arc, RwLock};

mod libs;
mod routers;

use libs::config::{
    create_default_config_file, get_tk_from_local_config, ServerConfig, StateConfig,
};
use libs::utils::create_cache_dir;
use routers::config::{auth, set_tokens, show_tokens};
use routers::docs::{docs, static_file};
use routers::geocloud::get_geocloud;
use routers::index::{favicon, index};
use routers::jilin1::{get_jl1, get_jl1earth};
use routers::wmts;

#[launch]
fn rocket() -> _ {
    println!("Tiny Tile Proxy\t v{}\n", env!("CARGO_PKG_VERSION"));
    create_default_config_file();
    create_cache_dir();

    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("config.toml").nested())
        .merge(Env::prefixed("ROCKET_").global());

    // 获取 tk 值
    let tk = get_tk_from_local_config().unwrap();

    let config: ServerConfig = figment.clone().extract().expect("Failed to extract config");

    let mut routers = routes![
        index,
        get_geocloud,
        get_jl1,
        get_jl1earth,
        docs,
        static_file,
        favicon,
        show_tokens,
        set_tokens,
        auth
    ];
    routers.extend(wmts::routers());

    rocket::custom(figment)
        .manage(StateConfig {
            tokens: Arc::new(RwLock::new(tk)),
            use_https: Arc::new(RwLock::new(config.use_https)),
        })
        .mount("/", routers)
}
