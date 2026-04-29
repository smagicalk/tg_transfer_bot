#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The show_message_sender setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventShowMessageSenderToggled {
    /// New value of show_message_sender
    pub show_message_sender: bool,
}
