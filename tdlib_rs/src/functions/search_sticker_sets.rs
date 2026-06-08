use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to return
/// * `query` - Query to search for
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_sticker_sets(
    sticker_type: crate::enums::StickerType,
    query: String,
    client_id: i32,
) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
    "@type": "searchStickerSets",
    "sticker_type": sticker_type,
    "query": query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
