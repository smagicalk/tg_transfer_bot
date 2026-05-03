use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a new sticker set. Returns the newly created sticker set
/// # Arguments
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `title` - Sticker set title; 1-64 characters
/// * `name` - Sticker set name. Can contain only English letters, digits and underscores. Must end with *"_by_<bot username>"* (*<bot_username>* is case insensitive) for bots; 0-64 characters.
/// If empty, then the name returned by getSuggestedStickerSetName will be used automatically
/// * `sticker_type` - Type of the stickers in the set
/// * `needs_repainting` - Pass true if stickers in the sticker set must be repainted; for custom emoji sticker sets only
/// * `stickers` - List of stickers to be added to the set; 1-200 stickers for custom emoji sticker sets, and 1-120 stickers otherwise. For TGS stickers, uploadStickerFile must be used before the sticker is shown
/// * `source` - Source of the sticker set; may be empty if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_new_sticker_set(
    user_id: i64,
    title: String,
    name: String,
    sticker_type: crate::enums::StickerType,
    needs_repainting: bool,
    stickers: Vec<crate::types::InputSticker>,
    source: String,
    client_id: i32,
) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
    "@type": "createNewStickerSet",
    "user_id": user_id,
    "title": title,
    "name": name,
    "sticker_type": sticker_type,
    "needs_repainting": needs_repainting,
    "stickers": stickers,
    "source": source,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
