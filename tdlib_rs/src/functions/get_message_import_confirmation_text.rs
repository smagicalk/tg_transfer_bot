#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a confirmation text to be shown to the user before starting message import
/// # Arguments
/// * `chat_id` - Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info member right
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_import_confirmation_text(chat_id: i64, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getMessageImportConfirmationText",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
