use minijinja::{context, Environment};
use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceMetadata {
    pub title: String,
    pub abstract_: String,
    pub keywords: Vec<String>,
}

#[derive(Serialize)]
pub struct Layer {
    pub title: String,
    pub abstract_: String,
    pub id: String,
    pub tile_matrix_set: String,
    pub url: String,
}

pub type Layers = Vec<Layer>;

#[derive(Debug, Clone, Serialize)]
pub struct TileMatrix {
    pub identifier: String,
    pub scale_denominator: f64,
    pub top_left_corner: (f64, f64),
    pub tile_width: u32,
    pub tile_height: u32,
    pub matrix_width: u32,
    pub matrix_height: u32,
}

#[derive(Debug, Serialize)]
pub struct TileMatrixSet {
    pub title: String,
    pub identifier: String,
    pub supported_crs: String,
    pub well_known_scale_set: String,
    pub tile_matrixs: Vec<TileMatrix>,
}

/// 生成瓦片矩阵集
/// ## 参数
/// - `min_zoom` - 起始级别
/// - `max_zoom` - 最大级别
/// ## Returns
/// 返回  TileMatrix 集合
pub fn generate_tile_matrixs(min_zoom: u32, max_zoom: u32) -> Vec<TileMatrix> {
    (min_zoom..=max_zoom)
        .map(|zoom| {
            // 精度问题只能存到小数后 7 位，因为前面整数已经有 9 位了
            let base_scale: f64 = 559_082_264.028_717_8;
            let scale = base_scale / 2f64.powi(zoom as i32);
            let matrix_size = 2u32.pow(zoom);

            TileMatrix {
                identifier: zoom.to_string(),
                scale_denominator: scale,
                top_left_corner: (-20037508.3427892, 20037508.3427892),
                tile_width: 256,
                tile_height: 256,
                matrix_width: matrix_size,
                matrix_height: matrix_size,
            }
        })
        .collect()
}

/// 生成 WebMercatorQuad 瓦片矩阵集
/// ## 参数
/// - `min_zoom` - 起始级别
/// - `max_zoom` - 最大级别
/// ### 参考
/// https://docs.ogc.org/is/17-083r4/17-083r4.html#toc49  
/// https://docs.ogc.org/is/17-083r2/17-083r2.html#72
pub fn get_web_mercator_quad_matrixs(min_zoom: u32, max_zoom: u32) -> TileMatrixSet {
    TileMatrixSet {
        title: "Google Maps Compatible for the World".to_string(),
        identifier: "WebMercatorQuad".to_string(),
        supported_crs: "urn:ogc:def:crs:EPSG:6.18.3:3857".to_string(),
        well_known_scale_set: "urn:ogc:def:wkss:OGC:1.0:GoogleMapsCompatible".to_string(),
        tile_matrixs: generate_tile_matrixs(min_zoom, max_zoom),
    }
}

/// 生成能力文档
/// ## 参数
/// - `service` - 服务元数据
/// - `layers` - 图层集合
/// - `tile_matrix_set` - 瓦片矩阵集
/// ## Returns
/// 返回能力文档字符串
pub fn generate_capabilities(
    service: &ServiceMetadata,
    layers: &Layers,
    tile_matrix_set: &TileMatrixSet,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_trim_blocks(true); // 自动删除块后的换行符
    env.set_lstrip_blocks(true); // 自动删除块前的空格

    env.add_template("base", include_str!("templates/base.jinja"))?;
    let template = env.get_template("base")?;
    let rendered = template.render(context! {
        service => service,
        layers => layers,
        tile_matrix_set => tile_matrix_set,
    })?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_metadata_rendering() {
        // 创建测试用的 ServiceMetadata
        let service = ServiceMetadata {
            title: "Test Service".to_string(),
            abstract_: "This is a test WMTS service".to_string(),
            keywords: vec!["test".to_string(), "wmts".to_string()],
        };

        let layer = Layer {
            title: "吉林一号".to_string(),
            abstract_: "This is a test WMTS layer".to_string(),
            id: "test".to_string(),
            tile_matrix_set: "WebMercatorQuad".to_string(),
            url: "http://localhost:8000".to_string(),
        };

        let layers = vec![layer];

        let tile_matrix_set = get_web_mercator_quad_matrixs(0, 18);

        // 生成 XML 能力文档
        let rendered = generate_capabilities(&service, &layers, &tile_matrix_set).unwrap();
        println!("{}", rendered);
        // 验证关键字段是否正确渲染
        assert!(rendered.contains("<ows:Title>Test Service</ows:Title>"));
        assert!(rendered.contains("<ows:Abstract>This is a test WMTS service</ows:Abstract>"));
        assert!(rendered.contains("<ows:Keyword>test</ows:Keyword>"));
        assert!(rendered.contains("<ows:Keyword>wmts</ows:Keyword>"));
    }

    #[test]
    fn test_matrix() {
        let matrix = get_web_mercator_quad_matrixs(0, 18);
        println!("{:?}", matrix);
    }
}
