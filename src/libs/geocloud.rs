use std::collections::HashMap;

use reqwest::Error;

fn create_parms(
    z: u32,
    x: u32,
    y: u32,
    layer: String,
    tk: String,
) -> HashMap<&'static str, String> {
    let mut params = HashMap::new();

    params.insert("tk", tk);
    params.insert("Width", "256".to_string());
    params.insert("Height", "256".to_string());
    params.insert("layer", layer);
    params.insert("style", "default".to_string());
    // tilematrixset 参数似乎不需要
    // params.insert("tilematrixset", format!("EPSG%3A4326_{}_028mm_GB", layer.clone));
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
    z: u32,
    x: u32,
    y: u32,
    layer: String,
    tk: String,
) -> Result<Vec<u8>, Error> {
    let url = format!(
        "https://igss.cgs.gov.cn:6160/igs/rest/ogc/{}/WMTSServer",
        layer
    );

    let params = create_parms(z, x, y, layer, tk);

    let client = reqwest::Client::builder().build()?;
    // 发送 GET 请求
    let response = client.get(url).query(&params).send().await?;
    let body = response.bytes().await?;
    Ok(body.to_vec())
}
