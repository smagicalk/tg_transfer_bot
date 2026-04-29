#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets a custom emoji sticker set thumbnail
/// # Arguments
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `custom_emoji_id` - Identifier of the custom emoji from the sticker set, which will be set as sticker set thumbnail; pass 0 to remove the sticker set thumbnail
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_emoji_sticker_set_thumbnail(name: String, custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCustomEmojiStickerSetThumbnail",
        "name": name,
        "custom_emoji_id": custom_emoji_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
