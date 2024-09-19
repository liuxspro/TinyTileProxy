pub mod geocloud;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_geocloud_tile() {
        let result =
            geocloud::get_geocloud_tile(4, 24, 5, "qg250w_20210416_ZAZSeOGX".to_string()).await;

        assert!(result.is_ok(), "get_geocloud_tile should return Ok");
        // 如果结果为 Ok，进一步验证返回的 body 是否符合预期
        if let Ok(body) = result {
            assert!(body.len() == 15301, "Body length should be 15301");
        }
    }
}
