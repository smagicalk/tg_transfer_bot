#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReplyMarkup {
    /// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, updateChatReplyMarkup with reply_markup_message == null will be sent
    #[serde(rename(
        serialize = "replyMarkupRemoveKeyboard",
        deserialize = "replyMarkupRemoveKeyboard"
    ))]
    RemoveKeyboard(crate::types::ReplyMarkupRemoveKeyboard),
    /// Instructs application to force a reply to this message
    #[serde(rename(
        serialize = "replyMarkupForceReply",
        deserialize = "replyMarkupForceReply"
    ))]
    ForceReply(crate::types::ReplyMarkupForceReply),
    /// Contains a custom keyboard layout to quickly reply to bots
    #[serde(rename(
        serialize = "replyMarkupShowKeyboard",
        deserialize = "replyMarkupShowKeyboard"
    ))]
    ShowKeyboard(crate::types::ReplyMarkupShowKeyboard),
    /// Contains an inline keyboard layout
    #[serde(rename(
        serialize = "replyMarkupInlineKeyboard",
        deserialize = "replyMarkupInlineKeyboard"
    ))]
    InlineKeyboard(crate::types::ReplyMarkupInlineKeyboard),
}
