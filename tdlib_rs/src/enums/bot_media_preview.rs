#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreview {
    /// Describes media previews of a bot
    #[serde(rename(serialize = "botMediaPreview", deserialize = "botMediaPreview"))]
    BotMediaPreview(crate::types::BotMediaPreview),
}
