#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether the specified bot can send messages to the user. Returns a 404 error if can't and the access can be granted by call to allowBotToSendMessages
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_bot_send_messages(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "canBotSendMessages",
        "bot_user_id": bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
