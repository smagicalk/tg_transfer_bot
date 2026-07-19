use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends 2-10 messages grouped together into an album on behalf of a business account; for bots only. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `chat_id` - Target chat
/// * `reply_to` - Information about the message to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving
/// * `effect_id` - Identifier of the effect to apply to the message
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_business_message_album(
    business_connection_id: String,
    chat_id: i64,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    disable_notification: bool,
    protect_content: bool,
    effect_id: i64,
    input_message_contents: Vec<crate::enums::InputMessageContent>,
    client_id: i32,
) -> Result<crate::enums::BusinessMessages, crate::types::Error> {
    let request = json!({
    "@type": "sendBusinessMessageAlbum",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "reply_to": reply_to,
    "disable_notification": disable_notification,
    "protect_content": protect_content,
    "effect_id": effect_id,
    "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
