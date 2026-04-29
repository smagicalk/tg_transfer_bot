#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolder {
    /// Represents a folder for user chats
    #[serde(rename(serialize = "chatFolder", deserialize = "chatFolder"))]
    ChatFolder(crate::types::ChatFolder),
}
