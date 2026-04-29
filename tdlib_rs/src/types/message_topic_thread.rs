#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A topic in a non-forum supergroup chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicThread {
    /// Unique identifier of the message thread
    pub message_thread_id: i64,
}
