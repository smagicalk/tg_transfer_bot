#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes accent color and background custom emoji for the current user; for Telegram Premium users only
/// # Arguments
/// * `accent_color_id` - Identifier of the accent color to use
/// * `background_custom_emoji_id` - Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_accent_color(accent_color_id: i32, background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAccentColor",
        "accent_color_id": accent_color_id,
        "background_custom_emoji_id": background_custom_emoji_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
