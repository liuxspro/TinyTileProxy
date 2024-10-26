pub mod geocloud;
pub mod jilin1;
pub mod utils;

#[cfg(test)]
mod tests {
    use utils::ZXY;

    use super::*;

    #[tokio::test]
    async fn get_geocloud_tile() {
        let tk = utils::get_tk_from_local_config().unwrap();
        let zxy = ZXY {
            z: "4".to_string(),
            x: 24,
            y: 5,
        };
        let result = geocloud::get_geocloud_tile(
            zxy,
            "qg250w_20210416_ZAZSeOGX".to_string(),
            tk.geocloud,
            None,
        )
        .await;

        assert!(result.is_ok(), "get_geocloud_tile should return Ok");
        // 如果结果为 Ok，进一步验证返回的 body 是否符合预期
        if let Ok(body) = result {
            assert!(body.len() == 15301, "Body length should be 15301");
        }
    }
}
