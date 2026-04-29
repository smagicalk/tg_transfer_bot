#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes application-specific data associated with a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_data` - New value of client_data
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_client_data(chat_id: i64, client_data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatClientData",
        "chat_id": chat_id,
        "client_data": client_data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
