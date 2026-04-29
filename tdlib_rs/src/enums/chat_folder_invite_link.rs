#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLink {
    /// Contains a chat folder invite link
    #[serde(rename(serialize = "chatFolderInviteLink", deserialize = "chatFolderInviteLink"))]
    ChatFolderInviteLink(crate::types::ChatFolderInviteLink),
}
