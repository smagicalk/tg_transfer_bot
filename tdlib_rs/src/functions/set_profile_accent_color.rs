#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes accent color and background custom emoji for profile of the current user; for Telegram Premium users only
/// # Arguments
/// * `profile_accent_color_id` - Identifier of the accent color to use for profile; pass -1 if none
/// * `profile_background_custom_emoji_id` - Identifier of a custom emoji to be shown on the user's profile photo background; 0 if none
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_profile_accent_color(profile_accent_color_id: i32, profile_background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setProfileAccentColor",
        "profile_accent_color_id": profile_accent_color_id,
        "profile_background_custom_emoji_id": profile_background_custom_emoji_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
