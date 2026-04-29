#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A poll in a message was stopped
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventPollStopped {
    /// The message with the poll
    pub message: crate::types::Message,
}
