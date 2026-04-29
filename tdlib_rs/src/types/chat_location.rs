#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a location to which a chat is connected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatLocation {
    /// The location
    pub location: crate::types::Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}
