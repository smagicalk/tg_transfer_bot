#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of chat members joined a chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkMembers {
    /// Approximate total number of chat members found
    pub total_count: i32,
    /// List of chat members, joined a chat via an invite link
    pub members: Vec<crate::types::ChatInviteLinkMember>,
}
