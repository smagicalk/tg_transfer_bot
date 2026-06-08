#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPosition {
    /// Describes a position of a chat in a chat list
    #[serde(rename(serialize = "chatPosition", deserialize = "chatPosition"))]
    ChatPosition(crate::types::ChatPosition),
}
