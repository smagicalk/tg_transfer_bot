#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message has been pinned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePinMessage {
    /// Identifier of the pinned message, can be an identifier of a deleted message or 0
    pub message_id: i64,
}
