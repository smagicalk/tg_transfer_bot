#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLinkInfo {
    /// Contains information about an invite link to a chat folder
    #[serde(rename(serialize = "chatFolderInviteLinkInfo", deserialize = "chatFolderInviteLinkInfo"))]
    ChatFolderInviteLinkInfo(crate::types::ChatFolderInviteLinkInfo),
}
