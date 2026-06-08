use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Asynchronously edits the text, media or caption of a quick reply message. Use quickReplyMessage.can_be_edited to check whether a message can be edited.
/// Media message can be edited only to a media message. Checklist messages can be edited only to a checklist message.
/// The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa
/// # Arguments
/// * `shortcut_id` - Unique identifier of the quick reply shortcut with the message
/// * `message_id` - Identifier of the message
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageChecklist, inputMessageDocument, inputMessagePhoto, inputMessageText, or inputMessageVideo
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_quick_reply_message(
    shortcut_id: i32,
    message_id: i64,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editQuickReplyMessage",
    "shortcut_id": shortcut_id,
    "message_id": message_id,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
