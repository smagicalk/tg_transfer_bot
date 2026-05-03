#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreviewInfo {
    /// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
    #[serde(rename(serialize = "botMediaPreviewInfo", deserialize = "botMediaPreviewInfo"))]
    BotMediaPreviewInfo(crate::types::BotMediaPreviewInfo),
}
