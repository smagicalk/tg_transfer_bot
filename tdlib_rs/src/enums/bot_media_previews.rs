#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreviews {
    /// Contains a list of media previews of a bot
    #[serde(rename(serialize = "botMediaPreviews", deserialize = "botMediaPreviews"))]
    BotMediaPreviews(crate::types::BotMediaPreviews),
}
