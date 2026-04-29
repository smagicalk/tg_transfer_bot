#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of messages from a business account as received by a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessMessages {
    /// List of business messages
    pub messages: Vec<crate::types::BusinessMessage>,
}
