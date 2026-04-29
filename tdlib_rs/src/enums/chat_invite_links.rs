#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinks {
    /// Contains a list of chat invite links
    #[serde(rename(serialize = "chatInviteLinks", deserialize = "chatInviteLinks"))]
    ChatInviteLinks(crate::types::ChatInviteLinks),
}
