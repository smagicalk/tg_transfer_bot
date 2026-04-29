#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for installed sticker sets by looking for specified query in their title and name
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to search for
/// * `query` - Query to search for
/// * `limit` - The maximum number of sticker sets to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_installed_sticker_sets(sticker_type: crate::enums::StickerType, query: String, limit: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "searchInstalledStickerSets",
        "sticker_type": sticker_type,
        "query": query,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
