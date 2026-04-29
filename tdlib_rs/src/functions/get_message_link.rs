#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTTPS link to a message in a chat. Available only if messageProperties.can_get_link, or if messageProperties.can_get_media_timestamp_links and a media timestamp link is generated. This is an offline method
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `media_timestamp` - If not 0, timestamp from which the video/audio/video note/voice note/story playing must start, in seconds. The media can be in the message content or in its link preview
/// * `for_album` - Pass true to create a link for the whole media album
/// * `in_message_thread` - Pass true to create a link to the message as a channel post comment, in a message thread, or a forum topic
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_link(chat_id: i64, message_id: i64, media_timestamp: i32, for_album: bool, in_message_thread: bool, client_id: i32) -> Result<crate::enums::MessageLink, crate::types::Error> {
    let request = json!({
        "@type": "getMessageLink",
        "chat_id": chat_id,
        "message_id": message_id,
        "media_timestamp": media_timestamp,
        "for_album": for_album,
        "in_message_thread": in_message_thread,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
