use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns outline of a sticker as an SVG path. This is an offline method. Returns an empty string if the outline isn't known
/// # Arguments
/// * `sticker_file_id` - File identifier of the sticker
/// * `for_animated_emoji` - Pass true to get the outline scaled for animated emoji
/// * `for_clicked_animated_emoji_message` - Pass true to get the outline scaled for clicked animated emoji message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_outline_svg_path(
    sticker_file_id: i32,
    for_animated_emoji: bool,
    for_clicked_animated_emoji_message: bool,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "getStickerOutlineSvgPath",
    "sticker_file_id": sticker_file_id,
    "for_animated_emoji": for_animated_emoji,
    "for_clicked_animated_emoji_message": for_clicked_animated_emoji_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
