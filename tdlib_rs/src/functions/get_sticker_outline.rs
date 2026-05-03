use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns outline of a sticker. This is an offline method. Returns a 404 error if the outline isn't known
/// # Arguments
/// * `sticker_file_id` - File identifier of the sticker
/// * `for_animated_emoji` - Pass true to get the outline scaled for animated emoji
/// * `for_clicked_animated_emoji_message` - Pass true to get the outline scaled for clicked animated emoji message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_outline(
    sticker_file_id: i32,
    for_animated_emoji: bool,
    for_clicked_animated_emoji_message: bool,
    client_id: i32,
) -> Result<crate::enums::Outline, crate::types::Error> {
    let request = json!({
    "@type": "getStickerOutline",
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
