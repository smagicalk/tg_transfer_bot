#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Chat {
    /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
    #[serde(rename(serialize = "chat", deserialize = "chat"))]
    Chat(crate::types::Chat),
}
