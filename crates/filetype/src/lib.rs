pub fn is_png(data: &[u8]) -> bool {
    const PNG_MAGIC_NUMBER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    data.len() >= 8 && data.starts_with(&PNG_MAGIC_NUMBER)
}

pub fn is_webp(data: &[u8]) -> bool {
    // 检查数据是否至少有 12 个字节
    if data.len() < 12 {
        return false;
    }

    // 使用 starts_with 避免显式索引
    if !data.starts_with(b"RIFF") {
        return false;
    }

    // 使用 get() 避免索引越界
    if let Some(file_type) = data.get(8..12) {
        if file_type == b"WEBP" {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_png() {
        let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let non_png_header = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x75];

        assert!(is_png(&png_header));
        assert!(!is_png(&non_png_header));
        assert!(!is_png(&[])); // 空数据
        assert!(!is_png(&png_header[..4])); // 数据不足 8 字节
    }

    #[test]
    fn test_is_webp() {
        let valid_webp: &[u8] = b"RIFF\x00\x00\x00\x00WEBP";
        let invalid_webp: &[u8] = b"RIFF\x00\x00\x00\x00JPEG";

        assert!(is_webp(valid_webp), "应该识别为 WebP");
        assert!(!is_webp(invalid_webp), "不应识别为 WebP");
        assert!(!is_webp(b""), "空数据不应是 WebP");
        assert!(!is_webp(b"RIFF"), "无效的 RIFF 数据");
    }
}
