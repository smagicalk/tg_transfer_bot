#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkInfo {
    /// Contains information about a chat invite link
    #[serde(rename(serialize = "chatInviteLinkInfo", deserialize = "chatInviteLinkInfo"))]
    ChatInviteLinkInfo(crate::types::ChatInviteLinkInfo),
}
