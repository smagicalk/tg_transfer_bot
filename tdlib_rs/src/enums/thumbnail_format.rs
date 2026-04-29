#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThumbnailFormat {
    /// The thumbnail is in JPEG format
    #[serde(rename(serialize = "thumbnailFormatJpeg", deserialize = "thumbnailFormatJpeg"))]
    Jpeg,
    /// The thumbnail is in static GIF format. It will be used only for some bot inline query results
    #[serde(rename(serialize = "thumbnailFormatGif", deserialize = "thumbnailFormatGif"))]
    Gif,
    /// The thumbnail is in MPEG4 format. It will be used only for some animations and videos
    #[serde(rename(serialize = "thumbnailFormatMpeg4", deserialize = "thumbnailFormatMpeg4"))]
    Mpeg4,
    /// The thumbnail is in PNG format. It will be used only for background patterns
    #[serde(rename(serialize = "thumbnailFormatPng", deserialize = "thumbnailFormatPng"))]
    Png,
    /// The thumbnail is in TGS format. It will be used only for sticker sets
    #[serde(rename(serialize = "thumbnailFormatTgs", deserialize = "thumbnailFormatTgs"))]
    Tgs,
    /// The thumbnail is in WEBM format. It will be used only for sticker sets
    #[serde(rename(serialize = "thumbnailFormatWebm", deserialize = "thumbnailFormatWebm"))]
    Webm,
    /// The thumbnail is in WEBP format. It will be used only for some stickers and sticker sets
    #[serde(rename(serialize = "thumbnailFormatWebp", deserialize = "thumbnailFormatWebp"))]
    Webp,
}
