#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a list of administrators of the chat with their custom titles
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_administrators(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatAdministrators, crate::types::Error> {
    let request = json!({
        "@type": "getChatAdministrators",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
