#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicChatType {
    /// The chat is public, because it has an active username
    #[serde(rename(serialize = "publicChatTypeHasUsername", deserialize = "publicChatTypeHasUsername"))]
    HasUsername,
    /// The chat is public, because it is a location-based supergroup
    #[serde(rename(serialize = "publicChatTypeIsLocationBased", deserialize = "publicChatTypeIsLocationBased"))]
    IsLocationBased,
}
