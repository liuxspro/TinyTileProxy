use std::collections::HashMap;

use reqwest::Error;

fn create_parms(
    z: &str,
    x: u32,
    y: u32,
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
    match tilematrixset {
        Some(tms) => {
            params.insert("tilematrixset", tms);
        }
        None => {}
    }

    params.insert("Service", "WMTS".to_string());
    params.insert("Request", "GetTile".to_string());
    params.insert("Version", "1.0.0".to_string());
    params.insert("Format", "image%2Fpng".to_string());
    params.insert("TileMatrix", z.to_string());
    params.insert("TileCol", x.to_string());
    params.insert("TileRow", y.to_string());

    return params;
}

pub async fn get_geocloud_tile(
    z: &str,
    x: u32,
    y: u32,
    layer: String,
    tk: String,
    tilematrixset: Option<String>,
) -> Result<Vec<u8>, Error> {
    let url = format!(
        "https://igss.cgs.gov.cn:6160/igs/rest/ogc/{}/WMTSServer",
        layer
    );

    let params = create_parms(z, x, y, layer, tk, tilematrixset);

    let client = reqwest::Client::builder().build()?;
    // 发送 GET 请求
    let response = client.get(url).query(&params).send().await?;
    // println!("{} ", response.url().as_str());
    let body = response.bytes().await?;
    // println!("{:?}", body);
    Ok(body.to_vec())
}
