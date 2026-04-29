#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits the message content of a checklist. Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `checklist` - The new checklist. If some tasks were completed, this information will be kept
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_checklist(chat_id: i64, message_id: i64, checklist: crate::types::InputChecklist, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageChecklist",
        "chat_id": chat_id,
        "message_id": message_id,
        "checklist": checklist,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
