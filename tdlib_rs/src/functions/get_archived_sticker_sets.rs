use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of archived sticker sets
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to return
/// * `offset_sticker_set_id` - Identifier of the sticker set from which to return the result; use 0 to get results from the beginning
/// * `limit` - The maximum number of sticker sets to return; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_archived_sticker_sets(
    sticker_type: crate::enums::StickerType,
    offset_sticker_set_id: i64,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
    "@type": "getArchivedStickerSets",
    "sticker_type": sticker_type,
    "offset_sticker_set_id": offset_sticker_set_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
