#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkCounts {
    /// Contains a list of chat invite link counts
    #[serde(rename(
        serialize = "chatInviteLinkCounts",
        deserialize = "chatInviteLinkCounts"
    ))]
    ChatInviteLinkCounts(crate::types::ChatInviteLinkCounts),
}
