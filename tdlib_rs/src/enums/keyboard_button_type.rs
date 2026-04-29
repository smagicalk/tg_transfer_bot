#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButtonType {
    /// A simple button, with text that must be sent when the button is pressed
    #[serde(rename(serialize = "keyboardButtonTypeText", deserialize = "keyboardButtonTypeText"))]
    Text,
    /// A button that sends the user's phone number when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestPhoneNumber", deserialize = "keyboardButtonTypeRequestPhoneNumber"))]
    RequestPhoneNumber,
    /// A button that sends the user's location when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestLocation", deserialize = "keyboardButtonTypeRequestLocation"))]
    RequestLocation,
    /// A button that allows the user to create and send a poll when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestPoll", deserialize = "keyboardButtonTypeRequestPoll"))]
    RequestPoll(crate::types::KeyboardButtonTypeRequestPoll),
    /// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
    #[serde(rename(serialize = "keyboardButtonTypeRequestUsers", deserialize = "keyboardButtonTypeRequestUsers"))]
    RequestUsers(crate::types::KeyboardButtonTypeRequestUsers),
    /// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
    #[serde(rename(serialize = "keyboardButtonTypeRequestChat", deserialize = "keyboardButtonTypeRequestChat"))]
    RequestChat(crate::types::KeyboardButtonTypeRequestChat),
    /// A button that opens a Web App by calling getWebAppUrl
    #[serde(rename(serialize = "keyboardButtonTypeWebApp", deserialize = "keyboardButtonTypeWebApp"))]
    WebApp(crate::types::KeyboardButtonTypeWebApp),
}
