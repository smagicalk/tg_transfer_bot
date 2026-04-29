#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether the bot can manage emoji status of the current user
/// # Arguments
/// * `bot_user_id` - User identifier of the bot
/// * `can_manage_emoji_status` - Pass true if the bot is allowed to change emoji status of the user; pass false otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_can_manage_emoji_status(bot_user_id: i64, can_manage_emoji_status: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotCanManageEmojiStatus",
        "bot_user_id": bot_user_id,
        "can_manage_emoji_status": can_manage_emoji_status,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
