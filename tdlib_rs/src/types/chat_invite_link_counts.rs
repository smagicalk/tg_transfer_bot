#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat invite link counts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkCounts {
    /// List of invite link counts
    pub invite_link_counts: Vec<crate::types::ChatInviteLinkCount>,
}
