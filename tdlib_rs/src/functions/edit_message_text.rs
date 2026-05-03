use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `input_message_content` - New text content of the message. Must be of type inputMessageText
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_text(
    chat_id: i64,
    message_id: i64,
    input_message_content: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "editMessageText",
    "chat_id": chat_id,
    "message_id": message_id,
    "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
