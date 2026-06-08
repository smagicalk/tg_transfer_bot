use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds 2-10 messages grouped together into an album to a quick reply shortcut. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
/// # Arguments
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_message_album(
    shortcut_name: String,
    reply_to_message_id: i64,
    input_message_contents: Vec<crate::enums::InputMessageContent>,
    client_id: i32,
) -> Result<crate::enums::QuickReplyMessages, crate::types::Error> {
    let request = json!({
    "@type": "addQuickReplyShortcutMessageAlbum",
    "shortcut_name": shortcut_name,
    "reply_to_message_id": reply_to_message_id,
    "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
