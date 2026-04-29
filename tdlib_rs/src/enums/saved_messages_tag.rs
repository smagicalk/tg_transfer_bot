#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTag {
    /// Represents a tag used in Saved Messages or a Saved Messages topic
    #[serde(rename(serialize = "savedMessagesTag", deserialize = "savedMessagesTag"))]
    SavedMessagesTag(crate::types::SavedMessagesTag),
}
