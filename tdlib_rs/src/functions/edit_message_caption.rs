#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits the message content caption. Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `caption` - New message content caption; 0-getOption("message_caption_length_max") characters; pass null to remove caption
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_caption(chat_id: i64, message_id: i64, caption: Option<crate::types::FormattedText>, show_caption_above_media: bool, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageCaption",
        "chat_id": chat_id,
        "message_id": message_id,
        "caption": caption,
        "show_caption_above_media": show_caption_above_media,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
