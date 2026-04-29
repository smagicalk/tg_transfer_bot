#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatType {
    /// An ordinary chat with a user
    #[serde(rename(serialize = "chatTypePrivate", deserialize = "chatTypePrivate"))]
    Private(crate::types::ChatTypePrivate),
    /// A basic group (a chat with 0-200 other users)
    #[serde(rename(serialize = "chatTypeBasicGroup", deserialize = "chatTypeBasicGroup"))]
    BasicGroup(crate::types::ChatTypeBasicGroup),
    /// A supergroup or channel (with unlimited members)
    #[serde(rename(serialize = "chatTypeSupergroup", deserialize = "chatTypeSupergroup"))]
    Supergroup(crate::types::ChatTypeSupergroup),
    /// A secret chat with a user
    #[serde(rename(serialize = "chatTypeSecret", deserialize = "chatTypeSecret"))]
    Secret(crate::types::ChatTypeSecret),
}
