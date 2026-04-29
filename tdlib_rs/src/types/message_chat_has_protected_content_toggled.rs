#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Chat has_protected_content setting was changed or request to change it was rejected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatHasProtectedContentToggled {
    /// Identifier of the message with the request to change the setting; can be an identifier of a deleted message or 0
    pub request_message_id: i64,
    /// Previous value of the setting
    pub old_has_protected_content: bool,
    /// New value of the setting
    pub new_has_protected_content: bool,
}
