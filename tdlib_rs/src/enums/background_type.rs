#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundType {
    /// A wallpaper in JPEG format
    #[serde(rename(serialize = "backgroundTypeWallpaper", deserialize = "backgroundTypeWallpaper"))]
    Wallpaper(crate::types::BackgroundTypeWallpaper),
    /// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
    #[serde(rename(serialize = "backgroundTypePattern", deserialize = "backgroundTypePattern"))]
    Pattern(crate::types::BackgroundTypePattern),
    /// A filled background
    #[serde(rename(serialize = "backgroundTypeFill", deserialize = "backgroundTypeFill"))]
    Fill(crate::types::BackgroundTypeFill),
    /// A background from a chat theme based on an emoji; can be used only as a chat background in channels
    #[serde(rename(serialize = "backgroundTypeChatTheme", deserialize = "backgroundTypeChatTheme"))]
    ChatTheme(crate::types::BackgroundTypeChatTheme),
}
