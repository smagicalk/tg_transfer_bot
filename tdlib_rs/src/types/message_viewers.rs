#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of message viewers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageViewers {
    /// List of message viewers
    pub viewers: Vec<crate::types::MessageViewer>,
}
