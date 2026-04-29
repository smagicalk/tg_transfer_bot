#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a message created with importMessages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageImportInfo {
    /// Name of the original sender
    pub sender_name: String,
    /// Point in time (Unix timestamp) when the message was originally sent
    pub date: i32,
}
