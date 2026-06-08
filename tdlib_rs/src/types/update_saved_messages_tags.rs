#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Tags used in Saved Messages or a Saved Messages topic have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTags {
    /// Identifier of Saved Messages topic which tags were changed; 0 if tags for the whole chat has changed
    pub saved_messages_topic_id: i64,
    /// The new tags
    pub tags: crate::types::SavedMessagesTags,
}
