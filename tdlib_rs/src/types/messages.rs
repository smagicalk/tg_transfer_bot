#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Messages {
    /// Approximate total number of messages found
    pub total_count: i32,
    /// List of messages; messages may be null
    pub messages: Vec<Option<crate::types::Message>>,
}
