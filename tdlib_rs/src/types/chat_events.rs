#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat events
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEvents {
    /// List of events
    pub events: Vec<crate::types::ChatEvent>,
}
