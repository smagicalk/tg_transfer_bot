#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a public forward as a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PublicForwardMessage {
    /// Information about the message
    pub message: crate::types::Message,
}
