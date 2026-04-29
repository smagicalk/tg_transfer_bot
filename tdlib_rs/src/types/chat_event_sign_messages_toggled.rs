#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The sign_messages setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventSignMessagesToggled {
    /// New value of sign_messages
    pub sign_messages: bool,
}
