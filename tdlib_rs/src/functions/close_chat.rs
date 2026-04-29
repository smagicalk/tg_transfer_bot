#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn close_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeChat",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
