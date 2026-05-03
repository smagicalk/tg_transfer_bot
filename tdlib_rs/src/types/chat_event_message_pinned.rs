#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message was pinned
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessagePinned {
    /// Pinned message
    pub message: crate::types::Message,
}
