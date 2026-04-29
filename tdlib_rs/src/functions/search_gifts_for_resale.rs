#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns upgraded gifts that can be bought from other owners using sendResoldGift
/// # Arguments
/// * `gift_id` - Identifier of the regular gift that was upgraded to a unique gift
/// * `order` - Order in which the results will be sorted
/// * `for_crafting` - Pass true to get only gifts suitable for crafting
/// * `attributes` - Attributes used to filter received gifts. If multiple attributes of the same type are specified, then all of them are allowed.
    /// If none attributes of specific type are specified, then all values for this attribute type are allowed
/// * `offset` - Offset of the first entry to return as received from the previous request with the same order and attributes; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_gifts_for_resale(gift_id: i64, order: crate::enums::GiftForResaleOrder, for_crafting: bool, attributes: Vec<crate::enums::UpgradedGiftAttributeId>, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::GiftsForResale, crate::types::Error> {
    let request = json!({
        "@type": "searchGiftsForResale",
        "gift_id": gift_id,
        "order": order,
        "for_crafting": for_crafting,
        "attributes": attributes,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
