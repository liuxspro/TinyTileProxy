use anyhow::Result as AnyhowResult;
/// 请求吉林1号瓦片
///
/// ## 参数
///
/// - `z` - z 值
/// - `x` - x 值
/// - `y` - y 值
/// - `mk` - 地图 mk
/// - `tk` - Token
/// ## Returns
/// 返回瓦片二进制数据 Result<Vec<u8>, reqwest::Error>
pub async fn get_tile(z: u32, x: u32, y: u32, mk: String, tk: String) -> AnyhowResult<Vec<u8>> {
    const AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    // 通过添加sch=wmts可返回正常XYZ顺序, 否则使用 `reversed_y: u32 = (1u32 << z) - 1 - y` 计算 -y 值
    let url = format!(
        "https://api.jl1mall.com/getMap/{}/{}/{}?mk={}&tk={}&sch=wmts",
        z, x, y, mk, tk
    );
    // 获取瓦片内容
    // 创建一个客户端并启用 Gzip 解压缩
    let client = reqwest::Client::builder()
        .user_agent(AGENT)
        .gzip(true)
        .build()?;
    // 发送 GET 请求
    let response = client.get(url).send().await?;
    let body = response.bytes().await?;
    // TODO tk不正确的时候也返回瓦片（参数有误），应返回错误
    Ok(body.to_vec())
}
