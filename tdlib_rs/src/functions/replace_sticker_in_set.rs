#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Replaces existing sticker in a set. The function is equivalent to removeStickerFromSet, then addStickerToSet, then setStickerPositionInSet
/// # Arguments
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `old_sticker` - Sticker to remove from the set
/// * `new_sticker` - Sticker to add to the set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn replace_sticker_in_set(user_id: i64, name: String, old_sticker: crate::enums::InputFile, new_sticker: crate::types::InputSticker, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "replaceStickerInSet",
        "user_id": user_id,
        "name": name,
        "old_sticker": old_sticker,
        "new_sticker": new_sticker,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
