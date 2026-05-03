#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkMembers {
    /// Contains a list of chat members joined a chat via an invite link
    #[serde(rename(
        serialize = "chatInviteLinkMembers",
        deserialize = "chatInviteLinkMembers"
    ))]
    ChatInviteLinkMembers(crate::types::ChatInviteLinkMembers),
}
