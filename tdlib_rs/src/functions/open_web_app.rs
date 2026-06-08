use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs TDLib that a Web App is being opened from the attachment menu, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an inlineKeyboardButtonTypeWebApp button.
/// For each bot, a confirmation alert about data sent to the bot must be shown once
/// # Arguments
/// * `chat_id` - Identifier of the chat in which the Web App is opened. The Web App can't be opened in secret chats
/// * `bot_user_id` - Identifier of the bot, providing the Web App. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `url` - The URL from an inlineKeyboardButtonTypeWebApp button, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an empty string otherwise
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied in the message sent by the Web App; pass null if none
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_web_app(
    chat_id: i64,
    bot_user_id: i64,
    url: String,
    topic_id: Option<crate::enums::MessageTopic>,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    parameters: crate::types::WebAppOpenParameters,
    client_id: i32,
) -> Result<crate::enums::WebAppInfo, crate::types::Error> {
    let request = json!({
    "@type": "openWebApp",
    "chat_id": chat_id,
    "bot_user_id": bot_user_id,
    "url": url,
    "topic_id": topic_id,
    "reply_to": reply_to,
    "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
