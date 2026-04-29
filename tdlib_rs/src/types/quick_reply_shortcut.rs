#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a shortcut that can be used for a quick reply
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyShortcut {
    /// Unique shortcut identifier
    pub id: i32,
    /// The name of the shortcut that can be used to use the shortcut
    pub name: String,
    /// The first shortcut message
    pub first_message: crate::types::QuickReplyMessage,
    /// The total number of messages in the shortcut
    pub message_count: i32,
}
