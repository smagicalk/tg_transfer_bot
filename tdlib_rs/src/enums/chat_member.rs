#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMember {
    /// Describes a user or a chat as a member of another chat
    #[serde(rename(serialize = "chatMember", deserialize = "chatMember"))]
    ChatMember(crate::types::ChatMember),
}
