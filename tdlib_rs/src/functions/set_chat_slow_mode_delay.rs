#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `slow_mode_delay` - New slow mode delay for the chat, in seconds; must be one of 0, 5, 10, 30, 60, 300, 900, 3600
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_slow_mode_delay(chat_id: i64, slow_mode_delay: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatSlowModeDelay",
        "chat_id": chat_id,
        "slow_mode_delay": slow_mode_delay,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
