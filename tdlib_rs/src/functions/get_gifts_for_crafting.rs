use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns upgraded gifts of the current user who can be used to craft another gifts
/// # Arguments
/// * `regular_gift_id` - Identifier of the regular gift that will be used for crafting
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gifts_for_crafting(
    regular_gift_id: i64,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::GiftsForCrafting, crate::types::Error> {
    let request = json!({
    "@type": "getGiftsForCrafting",
    "regular_gift_id": regular_gift_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
