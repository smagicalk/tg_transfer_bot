#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsInteractionInfo {
    /// Contains statistics about interactions with a message sent in the chat or a story posted on behalf of the chat
    #[serde(rename(
        serialize = "chatStatisticsInteractionInfo",
        deserialize = "chatStatisticsInteractionInfo"
    ))]
    ChatStatisticsInteractionInfo(crate::types::ChatStatisticsInteractionInfo),
}
