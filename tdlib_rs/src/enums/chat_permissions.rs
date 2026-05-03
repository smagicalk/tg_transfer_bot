#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPermissions {
    /// Describes actions that a user is allowed to take in a chat
    #[serde(rename(serialize = "chatPermissions", deserialize = "chatPermissions"))]
    ChatPermissions(crate::types::ChatPermissions),
}
