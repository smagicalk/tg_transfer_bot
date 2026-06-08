use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a new sticker to a set
/// # Arguments
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user, and contain less than 200 stickers for custom emoji sticker sets and less than 120 otherwise
/// * `sticker` - Sticker to add to the set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_sticker_to_set(
    user_id: i64,
    name: String,
    sticker: crate::types::InputSticker,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addStickerToSet",
    "user_id": user_id,
    "name": name,
    "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
