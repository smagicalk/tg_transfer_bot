#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes accent color and background custom emoji of a channel chat. Requires can_change_info administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `accent_color_id` - Identifier of the accent color to use. The chat must have at least accentColor.min_channel_chat_boost_level boost level to pass the corresponding color
/// * `background_custom_emoji_id` - Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none. Use chatBoostLevelFeatures.can_set_background_custom_emoji to check whether a custom emoji can be set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_accent_color(chat_id: i64, accent_color_id: i32, background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAccentColor",
        "chat_id": chat_id,
        "accent_color_id": accent_color_id,
        "background_custom_emoji_id": background_custom_emoji_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
