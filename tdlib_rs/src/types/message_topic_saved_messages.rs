#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A topic in Saved Messages chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicSavedMessages {
    /// Unique identifier of the Saved Messages topic
    pub saved_messages_topic_id: i64,
}
