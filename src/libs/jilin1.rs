use reqwest::Error;

pub async fn get_jl_tile(z: u32, x: u32, y: u32, mk: String, tk: String) -> Result<Vec<u8>, Error> {
    const AGENT:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    let url = format!(
        "https://api.jl1mall.com/getMap/{}/{}/{}?mk={}&tk={}",
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
    Ok(body.to_vec())
}
