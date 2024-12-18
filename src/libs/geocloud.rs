use anyhow::Result as AnyhowResult;
use reqwest::Error;
use std::collections::HashMap;
use std::fs::create_dir_all;

use super::utils::{get_cache_dir, get_map_names, read_file, save_png, ZXY};

fn get_z_value(zxy: &ZXY) -> String {
    let z = &zxy.z;
    if z.as_str().len() <= 2 {
        z.to_string()
    } else {
        let z_slice: &str = z;
        let parts: Vec<&str> = z_slice.split(':').collect();
        parts.last().unwrap().to_string()
    }
}

fn create_parms(
    zxy: ZXY,
    layer: String,
    tk: String,
    tilematrixset: Option<String>,
) -> HashMap<&'static str, String> {
    let mut params = HashMap::new();
    params.insert("tk", tk);
    params.insert("Width", "256".to_string());
    params.insert("Height", "256".to_string());
    params.insert("layer", layer);
    // params.insert("style", "default".to_string());
    // tilematrixset EPSG%:4326_{layer}_028mm_GB / EPSG:4326
    if let Some(tms) = tilematrixset {
        params.insert("tilematrixset", tms);
    }

    params.insert("Service", "WMTS".to_string());
    params.insert("Request", "GetTile".to_string());
    params.insert("Version", "1.0.0".to_string());
    params.insert("Format", "image%2Fpng".to_string());
    params.insert("TileMatrix", zxy.z);
    params.insert("TileCol", zxy.x.to_string());
    params.insert("TileRow", zxy.y.to_string());

    params
}

pub async fn get_geocloud_tile(
    zxy: ZXY,
    layer: String,
    tk: String,
    tilematrixset: Option<String>,
) -> Result<Vec<u8>, Error> {
    let url = format!(
        "https://igss.cgs.gov.cn:6160/igs/rest/ogc/{}/WMTSServer",
        layer
    );

    let params = create_parms(zxy, layer, tk, tilematrixset);

    let client = reqwest::Client::builder().build()?;
    // 发送 GET 请求
    let response = client.get(url).query(&params).send().await?;
    // println!("{} ", response.url().as_str());
    let body = response.bytes().await?;
    // println!("{:?}", body);
    Ok(body.to_vec())
}

pub async fn get_geocloud_tile_cache(
    zxy: ZXY,
    layer: String,
    tk: String,
    tilematrixset: Option<String>,
) -> AnyhowResult<Vec<u8>> {
    let map_names = get_map_names();
    let cache_dir = get_cache_dir();
    let map_dir = cache_dir.join(format!(
        "Geocloud/{}",
        map_names.get(layer.as_str()).unwrap_or(&layer.as_str())
    ));
    let tile_dir = map_dir.join(format!("{}/{}/", get_z_value(&zxy), zxy.x));
    let tile_path = tile_dir.join(format!("{}.png", zxy.y));
    if tile_path.exists() {
        let png_data = read_file(&tile_path)?;
        Ok(png_data)
    } else {
        create_dir_all(&tile_dir).expect("Filed to create Tile Dir");
        match get_geocloud_tile(zxy, layer, tk, tilematrixset).await {
            Ok(body) => {
                save_png(tile_path, &body)?;
                Ok(body)
            }
            Err(e) => Err(e.into()),
        }
    }
}
