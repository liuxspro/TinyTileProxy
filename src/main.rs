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
use routers::geocloud::get_geocloud;
use routers::index::index;
use routers::jilin1::get_jl1;
use routers::wmts::get_geocloud_wmts;

#[launch]
fn rocket() -> _ {
    create_default_config_file().unwrap();

    println!("Tiny Tile Proxy\n");

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
    if tk.jl1 == "" {
        panic!("Error: jilin1 tk not set");
    }

    println!("Server will be running at http://{}:{}\n", address, port);
    rocket::custom(figment)
        .manage(ServerConfig {
            ip: address.to_string(),
            port,
            tokens: tk,
        })
        .mount(
            "/",
            routes![index, get_geocloud, get_jl1, get_geocloud_wmts],
        )
}
