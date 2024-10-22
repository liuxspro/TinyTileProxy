#[macro_use]
extern crate rocket;

use figment::Figment;
use rocket::Config;
// use std::io::{self, Write};

mod libs;
mod routers;

use libs::utils::{
    create_cache_dir, create_default_config_file, get_local_config_data, get_local_ip,
    get_tk_from_local_config, is_unspecified, ServerConfig,
};
use routers::docs::{docs, static_file};
use routers::geocloud::get_geocloud;
use routers::index::{favicon, index};
use routers::jilin1::get_jl1;
use routers::wmts;

#[launch]
fn rocket() -> _ {
    create_default_config_file().unwrap();
    create_cache_dir();

    println!("Tiny Tile Proxy\t v{}\n", env!("CARGO_PKG_VERSION"));

    let local_config = get_local_config_data();
    let figment = Figment::from(rocket::Config::default()).merge(local_config.nested());

    let config: Config = figment.extract().expect("Failed to extract config");

    // 获取 IP 及端口
    let mut address = config.address;
    let port = config.port;
    if is_unspecified(address) {
        match get_local_ip() {
            Some(ip) => {
                address = ip;
            }
            None => {
                eprintln!("Filed to get ip");
            }
        }
    }

    // 获取 tk 值
    let tk = get_tk_from_local_config().unwrap();

    // 检查tk值是否为空
    if tk.jl1.is_empty() {
        eprintln!("吉林一号 tk 值未设置,请在 config.toml 中输入 tk 后重新运行...\n");
        // io::stdout().flush().unwrap();
        // let _ = io::stdin().read_line(&mut String::new());
        // std::process::exit(0);
    }

    // println!("Server will be running at http://{}:{}\n", address, port);
    println!("访问: http://{}:{} 查看使用方法\n", address, port);

    let mut routers = routes![index, get_geocloud, get_jl1, docs, static_file, favicon];
    routers.extend(wmts::routers());

    rocket::custom(figment)
        .manage(ServerConfig {
            ip: address.to_string(),
            port,
            tokens: tk,
        })
        .mount("/", routers)
}
