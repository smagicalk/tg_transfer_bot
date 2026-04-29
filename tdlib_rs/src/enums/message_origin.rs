#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageOrigin {
    /// The message was originally sent by a known user
    #[serde(rename(serialize = "messageOriginUser", deserialize = "messageOriginUser"))]
    User(crate::types::MessageOriginUser),
    /// The message was originally sent by a user, which is hidden by their privacy settings
    #[serde(rename(serialize = "messageOriginHiddenUser", deserialize = "messageOriginHiddenUser"))]
    HiddenUser(crate::types::MessageOriginHiddenUser),
    /// The message was originally sent on behalf of a chat
    #[serde(rename(serialize = "messageOriginChat", deserialize = "messageOriginChat"))]
    Chat(crate::types::MessageOriginChat),
    /// The message was originally a post in a channel
    #[serde(rename(serialize = "messageOriginChannel", deserialize = "messageOriginChannel"))]
    Channel(crate::types::MessageOriginChannel),
}
