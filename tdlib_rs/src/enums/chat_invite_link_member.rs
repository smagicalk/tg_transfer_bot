#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkMember {
    /// Describes a chat member joined a chat via an invite link
    #[serde(rename(serialize = "chatInviteLinkMember", deserialize = "chatInviteLinkMember"))]
    ChatInviteLinkMember(crate::types::ChatInviteLinkMember),
}
