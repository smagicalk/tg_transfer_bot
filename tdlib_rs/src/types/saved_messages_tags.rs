#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of tags used in Saved Messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTags {
    /// List of tags
    pub tags: Vec<crate::types::SavedMessagesTag>,
}
