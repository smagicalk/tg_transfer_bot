#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of message senders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenders {
    /// Approximate total number of message senders found
    pub total_count: i32,
    /// List of message senders
    pub senders: Vec<crate::enums::MessageSender>,
}
