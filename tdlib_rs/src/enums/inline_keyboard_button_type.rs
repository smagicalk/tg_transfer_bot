#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineKeyboardButtonType {
    /// A button that opens a specified URL
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeUrl",
        deserialize = "inlineKeyboardButtonTypeUrl"
    ))]
    Url(crate::types::InlineKeyboardButtonTypeUrl),
    /// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeLoginUrl",
        deserialize = "inlineKeyboardButtonTypeLoginUrl"
    ))]
    LoginUrl(crate::types::InlineKeyboardButtonTypeLoginUrl),
    /// A button that opens a Web App by calling openWebApp
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeWebApp",
        deserialize = "inlineKeyboardButtonTypeWebApp"
    ))]
    WebApp(crate::types::InlineKeyboardButtonTypeWebApp),
    /// A button that sends a callback query to a bot
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallback",
        deserialize = "inlineKeyboardButtonTypeCallback"
    ))]
    Callback(crate::types::InlineKeyboardButtonTypeCallback),
    /// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallbackWithPassword",
        deserialize = "inlineKeyboardButtonTypeCallbackWithPassword"
    ))]
    CallbackWithPassword(crate::types::InlineKeyboardButtonTypeCallbackWithPassword),
    /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallbackGame",
        deserialize = "inlineKeyboardButtonTypeCallbackGame"
    ))]
    CallbackGame,
    /// A button that forces an inline query to the bot to be inserted in the input field
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeSwitchInline",
        deserialize = "inlineKeyboardButtonTypeSwitchInline"
    ))]
    SwitchInline(crate::types::InlineKeyboardButtonTypeSwitchInline),
    /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeBuy",
        deserialize = "inlineKeyboardButtonTypeBuy"
    ))]
    Buy,
    /// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeUser",
        deserialize = "inlineKeyboardButtonTypeUser"
    ))]
    User(crate::types::InlineKeyboardButtonTypeUser),
    /// A button that copies specified text to clipboard
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCopyText",
        deserialize = "inlineKeyboardButtonTypeCopyText"
    ))]
    CopyText(crate::types::InlineKeyboardButtonTypeCopyText),
}
