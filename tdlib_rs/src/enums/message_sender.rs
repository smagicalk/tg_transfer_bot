#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSender {
    /// The message was sent by a known user
    #[serde(rename(serialize = "messageSenderUser", deserialize = "messageSenderUser"))]
    User(crate::types::MessageSenderUser),
    /// The message was sent on behalf of a chat
    #[serde(rename(serialize = "messageSenderChat", deserialize = "messageSenderChat"))]
    Chat(crate::types::MessageSenderChat),
}
