#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a tag used in Saved Messages or a Saved Messages topic
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTag {
    /// The tag
    pub tag: crate::enums::ReactionType,
    /// Label of the tag; 0-12 characters. Always empty if the tag is returned for a Saved Messages topic
    pub label: String,
    /// Number of times the tag was used; may be 0 if the tag has non-empty label
    pub count: i32,
}
