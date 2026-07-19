#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new message was added to a business account; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewBusinessMessage {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// The new message
    pub message: crate::types::BusinessMessage,
}
