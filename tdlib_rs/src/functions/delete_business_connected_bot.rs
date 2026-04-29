#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes the business bot that is connected to the current user account
/// # Arguments
/// * `bot_user_id` - Unique user identifier for the bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_connected_bot(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessConnectedBot",
        "bot_user_id": bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
