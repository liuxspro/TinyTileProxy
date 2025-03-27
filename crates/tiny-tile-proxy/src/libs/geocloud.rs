use anyhow::Result as AnyhowResult;
use reqwest::Error;
use std::collections::HashMap;
use std::fs::create_dir_all;

use super::utils::{get_cache_dir, read_file, save_png, ZXY};

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

pub fn get_map_names() -> HashMap<&'static str, &'static str> {
    // 地质云 服务接口 https://igss.cgs.gov.cn/admin/token/index.jsp
    // 授权范围内一共 10 个服务
    // 其中有 2 个为重复(使用 web 墨卡托投影)
    let mut map_names = HashMap::new();
    map_names.insert("qg20_20210401_FCnDDRJd", "全国1比20万地质图空间数据库");
    map_names.insert("qg50w_20210416_F7qGy9A7", "全国1比50万地质图数据");
    map_names.insert(
        "全国100万地质图_20210330_rpam5kdJ",
        "全国1比100万地质图空间数据",
    );
    map_names.insert("qg150w_20210416_BIwqE0wU", "全国1比150万地质图数据");
    map_names.insert("qg250w_20210416_ZAZSeOGX", "全国1比250万地质图数据");
    map_names.insert(
        "gisdatamanageCZL:zgdzhjfqt2015556_20220902_jFEtRvYd",
        "中国地质环境分区图",
    );
    map_names.insert(
        "gisdatamanageCZL:hhhpytrxhjt24172036_20220905_bAcMBmmq",
        "黄淮海平原土壤硒环境图",
    );
    map_names.insert(
        "gisdatamanageCZL:0715zgdzhjaqcdt17155125_20220905_ejCKPwC4",
        "中国地质环境安全程度图",
    );
    // 这两个不是授权范围内的
    // map_names.insert(
    //     "gisdatamanageCZL:500wdxszyt2616300_20220905_BhR4tgbF",
    //     "中国地下水资源图",
    // );
    // map_names.insert(
    //     "gisdatamanageCZL:zgswdzt16175436_20220905_BbcQipWD",
    //     "中国水文地质图",
    // );

    map_names
}
