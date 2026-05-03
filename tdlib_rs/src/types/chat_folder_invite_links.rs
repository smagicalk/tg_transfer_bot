#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of chat folder invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLinks {
    /// List of the invite links
    pub invite_links: Vec<crate::types::ChatFolderInviteLink>,
}
