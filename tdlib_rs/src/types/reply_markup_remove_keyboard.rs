#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, updateChatReplyMarkup with reply_markup_message == null will be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupRemoveKeyboard {
    /// True, if the keyboard is removed only for the mentioned users or the target user of a reply
    pub is_personal: bool,
}
