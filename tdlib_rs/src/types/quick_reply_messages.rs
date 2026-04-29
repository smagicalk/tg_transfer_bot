#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of quick reply messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyMessages {
    /// List of quick reply messages; messages may be null
    pub messages: Vec<Option<crate::types::QuickReplyMessage>>,
}
