use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes accent color and background custom emoji for profile of a supergroup or channel chat. Requires can_change_info administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `profile_accent_color_id` - Identifier of the accent color to use for profile; pass -1 if none. The chat must have at least profileAccentColor.min_supergroup_chat_boost_level for supergroups
/// or profileAccentColor.min_channel_chat_boost_level for channels boost level to pass the corresponding color
/// * `profile_background_custom_emoji_id` - Identifier of a custom emoji to be shown on the chat's profile photo background; 0 if none. Use chatBoostLevelFeatures.can_set_profile_background_custom_emoji to check whether a custom emoji can be set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_profile_accent_color(
    chat_id: i64,
    profile_accent_color_id: i32,
    profile_background_custom_emoji_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatProfileAccentColor",
    "chat_id": chat_id,
    "profile_accent_color_id": profile_accent_color_id,
    "profile_background_custom_emoji_id": profile_background_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
