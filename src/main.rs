#[macro_use]
extern crate rocket;

use figment::Figment;
use rocket::Config;

mod libs;
mod routers;

use libs::utils::{
    create_default_config_file, get_local_config_data, get_local_ip, get_tk_from_local_config,
    is_unspecified, ServerConfig,
};
use routers::docs::{docs, static_file};
use routers::geocloud::get_geocloud;
use routers::index::index;
use routers::jilin1::get_jl1;
use routers::wmts::{get_geocloud_wmts, get_jl1_wmts, get_xyz_wmts};

#[launch]
fn rocket() -> _ {
    create_default_config_file().unwrap();

    println!("Tiny Tile Proxy\t Version: {}\n", env!("CARGO_PKG_VERSION"));

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
    // println!("Server will be running at http://{}:{}\n", address, port);
    println!("使用浏览器访问: http://{}:{} 查看使用方法\n", address, port);
    rocket::custom(figment)
        .manage(ServerConfig {
            ip: address.to_string(),
            port,
            tokens: tk,
        })
        .mount(
            "/",
            routes![
                index,
                get_geocloud,
                get_jl1,
                get_geocloud_wmts,
                docs,
                static_file,
                get_jl1_wmts,
                get_xyz_wmts,
            ],
        )
}
