#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLink {
    /// Contains a chat invite link
    #[serde(rename(serialize = "chatInviteLink", deserialize = "chatInviteLink"))]
    ChatInviteLink(crate::types::ChatInviteLink),
}
