#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first.
/// Only stickers belonging to a sticker set or in WEBP or WEBM format can be added to this list. Emoji stickers can't be added to favorite stickers
/// # Arguments
/// * `sticker` - Sticker file to add
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_favorite_sticker(sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addFavoriteSticker",
        "sticker": sticker,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
