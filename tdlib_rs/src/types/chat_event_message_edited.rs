#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message was edited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageEdited {
    /// The original message before the edit
    pub old_message: crate::types::Message,
    /// The message after it was edited
    pub new_message: crate::types::Message,
}
