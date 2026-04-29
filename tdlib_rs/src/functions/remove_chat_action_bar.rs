#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a chat action bar without any other action
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_chat_action_bar(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeChatActionBar",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
