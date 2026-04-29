#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProfileTab {
    /// A tab with stories posted by the user or the channel chat and saved to profile
    #[serde(rename(serialize = "profileTabPosts", deserialize = "profileTabPosts"))]
    Posts,
    /// A tab with gifts received by the user or the channel chat
    #[serde(rename(serialize = "profileTabGifts", deserialize = "profileTabGifts"))]
    Gifts,
    /// A tab with photos and videos posted by the channel
    #[serde(rename(serialize = "profileTabMedia", deserialize = "profileTabMedia"))]
    Media,
    /// A tab with documents posted by the channel
    #[serde(rename(serialize = "profileTabFiles", deserialize = "profileTabFiles"))]
    Files,
    /// A tab with messages posted by the channel and containing links
    #[serde(rename(serialize = "profileTabLinks", deserialize = "profileTabLinks"))]
    Links,
    /// A tab with audio messages posted by the channel
    #[serde(rename(serialize = "profileTabMusic", deserialize = "profileTabMusic"))]
    Music,
    /// A tab with voice notes posted by the channel
    #[serde(rename(serialize = "profileTabVoice", deserialize = "profileTabVoice"))]
    Voice,
    /// A tab with animations posted by the channel
    #[serde(rename(serialize = "profileTabGifs", deserialize = "profileTabGifs"))]
    Gifs,
}
