#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A background previously set in the chat; for chat backgrounds only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundPrevious {
    /// Identifier of the message with the background
    pub message_id: i64,
}
