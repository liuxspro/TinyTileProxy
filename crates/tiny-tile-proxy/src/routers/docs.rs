use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::Embed;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "../../docs/.vitepress/dist"]
struct Asset;

#[get("/docs")]
pub fn docs() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

#[get("/docs/<file..>")]
pub fn static_file(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);
    Some((content_type, asset.data))
}
