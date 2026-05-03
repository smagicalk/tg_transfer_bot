#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of sponsored messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredMessages {
    /// List of sponsored messages
    pub messages: Vec<crate::types::SponsoredMessage>,
    /// The minimum number of messages between shown sponsored messages, or 0 if only one sponsored message must be shown after all ordinary messages
    pub messages_between: i32,
}
