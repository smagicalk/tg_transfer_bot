#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkCount {
    /// Describes a chat administrator with a number of active and revoked chat invite links
    #[serde(rename(serialize = "chatInviteLinkCount", deserialize = "chatInviteLinkCount"))]
    ChatInviteLinkCount(crate::types::ChatInviteLinkCount),
}
