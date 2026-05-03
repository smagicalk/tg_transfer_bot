use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the custom emoji sticker set of a supergroup; requires can_change_info administrator right. The chat must have at least chatBoostFeatures.min_custom_emoji_sticker_set_boost_level boost level to pass the corresponding color
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `custom_emoji_sticker_set_id` - New value of the custom emoji sticker set identifier for the supergroup. Use 0 to remove the custom emoji sticker set in the supergroup
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_custom_emoji_sticker_set(
    supergroup_id: i64,
    custom_emoji_sticker_set_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setSupergroupCustomEmojiStickerSet",
    "supergroup_id": supergroup_id,
    "custom_emoji_sticker_set_id": custom_emoji_sticker_set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
