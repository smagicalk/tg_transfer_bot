use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for a sticker set by its name
/// # Arguments
/// * `name` - Name of the sticker set
/// * `ignore_cache` - Pass true to ignore local cache of sticker sets and always send a network request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_sticker_set(
    name: String,
    ignore_cache: bool,
    client_id: i32,
) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
    "@type": "searchStickerSet",
    "name": name,
    "ignore_cache": ignore_cache,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
