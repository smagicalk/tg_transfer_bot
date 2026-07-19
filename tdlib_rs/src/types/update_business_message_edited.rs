#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message in a business account was edited; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessMessageEdited {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// The edited message
    pub message: crate::types::BusinessMessage,
}
