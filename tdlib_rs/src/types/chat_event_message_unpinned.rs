#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message was unpinned
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageUnpinned {
    /// Unpinned message
    pub message: crate::types::Message,
}
