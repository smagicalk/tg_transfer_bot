use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of trending sticker sets. For optimal performance, the number of returned sticker sets is chosen by TDLib
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to return
/// * `offset` - The offset from which to return the sticker sets; must be non-negative
/// * `limit` - The maximum number of sticker sets to be returned; up to 100. For optimal performance, the number of returned sticker sets is chosen by TDLib and can be smaller than the specified limit, even if the end of the list has not been reached
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_trending_sticker_sets(
    sticker_type: crate::enums::StickerType,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::TrendingStickerSets, crate::types::Error> {
    let request = json!({
    "@type": "getTrendingStickerSets",
    "sticker_type": sticker_type,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
