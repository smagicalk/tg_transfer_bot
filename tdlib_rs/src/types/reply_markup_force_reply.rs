#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Instructs application to force a reply to this message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupForceReply {
    /// True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply
    pub is_personal: bool,
    /// If non-empty, the placeholder to be shown in the input field when the reply is active; 0-64 characters
    pub input_field_placeholder: String,
}
