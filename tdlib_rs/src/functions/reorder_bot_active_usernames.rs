use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes order of active usernames of a bot. Can be called only if userTypeBot.can_be_edited == true
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_bot_active_usernames(
    bot_user_id: i64,
    usernames: Vec<String>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "reorderBotActiveUsernames",
    "bot_user_id": bot_user_id,
    "usernames": usernames,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
