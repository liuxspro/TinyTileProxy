use figment::{
    providers::{Format, Toml},
    Figment,
};

pub fn get_tk_from_local_config() -> String {
    let config = Figment::from(get_local_config_data());
    config
        .extract_inner("geocloud.tk")
        .expect("Filed to get geocloud tk")
}

pub fn get_local_config_data() -> figment::providers::Data<Toml> {
    Toml::file("config.toml")
}
