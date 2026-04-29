#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveLiveLocationMessages {
    /// The list of messages with active live locations
    pub messages: Vec<crate::types::Message>,
}
