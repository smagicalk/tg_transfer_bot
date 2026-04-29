#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the background in a specific chat. Supported only in private and secret chats with non-deleted users, and in chats with sufficient boost level and can_change_info administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `background` - The input background to use; pass null to create a new filled or chat theme background
/// * `r#type` - Background type; pass null to use default background type for the chosen background; backgroundTypeChatTheme isn't supported for private and secret chats.
    /// Use chatBoostLevelFeatures.chat_theme_background_count and chatBoostLevelFeatures.can_set_custom_background to check whether the background type can be set in the boosted chat
/// * `dark_theme_dimming` - Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
/// * `only_for_self` - Pass true to set background only for self; pass false to set background for all chat users. Always false for backgrounds set in boosted chats. Background can be set for both users only by Telegram Premium users and if set background isn't of the type inputBackgroundPrevious
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_background(chat_id: i64, background: Option<crate::enums::InputBackground>, r#type: Option<crate::enums::BackgroundType>, dark_theme_dimming: i32, only_for_self: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatBackground",
        "chat_id": chat_id,
        "background": background,
        "type": r#type,
        "dark_theme_dimming": dark_theme_dimming,
        "only_for_self": only_for_self,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
