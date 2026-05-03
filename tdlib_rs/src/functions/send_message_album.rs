use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends 2-10 messages grouped together into an album. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
/// # Arguments
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the messages will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the messages; pass null to use default options
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_message_album(
    chat_id: i64,
    topic_id: Option<crate::enums::MessageTopic>,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    options: Option<crate::types::MessageSendOptions>,
    input_message_contents: Vec<crate::enums::InputMessageContent>,
    client_id: i32,
) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
    "@type": "sendMessageAlbum",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "reply_to": reply_to,
    "options": options,
    "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
