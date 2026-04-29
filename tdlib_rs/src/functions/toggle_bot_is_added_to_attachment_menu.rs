#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds or removes a bot to attachment and side menu. Bot can be added to the menu, only if userTypeBot.can_be_added_to_attachment_menu == true
/// # Arguments
/// * `bot_user_id` - Bot's user identifier
/// * `is_added` - Pass true to add the bot to attachment menu; pass false to remove the bot from attachment menu
/// * `allow_write_access` - Pass true if the current user allowed the bot to send them messages. Ignored if is_added is false
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_is_added_to_attachment_menu(bot_user_id: i64, is_added: bool, allow_write_access: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotIsAddedToAttachmentMenu",
        "bot_user_id": bot_user_id,
        "is_added": is_added,
        "allow_write_access": allow_write_access,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
