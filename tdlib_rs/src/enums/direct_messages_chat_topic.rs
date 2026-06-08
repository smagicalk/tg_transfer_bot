#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DirectMessagesChatTopic {
    /// Contains information about a topic in a channel direct messages chat administered by the current user
    #[serde(rename(
        serialize = "directMessagesChatTopic",
        deserialize = "directMessagesChatTopic"
    ))]
    DirectMessagesChatTopic(crate::types::DirectMessagesChatTopic),
}
