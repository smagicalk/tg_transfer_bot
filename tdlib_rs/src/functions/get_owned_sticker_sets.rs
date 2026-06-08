use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns sticker sets owned by the current user
/// # Arguments
/// * `offset_sticker_set_id` - Identifier of the sticker set from which to return owned sticker sets; use 0 to get results from the beginning
/// * `limit` - The maximum number of sticker sets to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_owned_sticker_sets(
    offset_sticker_set_id: i64,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
    "@type": "getOwnedStickerSets",
    "offset_sticker_set_id": offset_sticker_set_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
