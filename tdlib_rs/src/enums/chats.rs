#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Chats {
    /// Represents a list of chats
    #[serde(rename(serialize = "chats", deserialize = "chats"))]
    Chats(crate::types::Chats),
}
