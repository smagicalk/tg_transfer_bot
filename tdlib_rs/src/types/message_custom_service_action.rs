#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A non-standard action has happened in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCustomServiceAction {
    /// Message text to be shown in the chat
    pub text: String,
}
