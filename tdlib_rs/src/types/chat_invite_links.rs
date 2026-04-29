#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinks {
    /// Approximate total number of chat invite links found
    pub total_count: i32,
    /// List of invite links
    pub invite_links: Vec<crate::types::ChatInviteLink>,
}
