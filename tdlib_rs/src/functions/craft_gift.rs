#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Crafts a new gift from other gifts that will be permanently lost
/// # Arguments
/// * `received_gift_ids` - Identifier of the gifts to use for crafting. In the case of a successful craft, the resulting gift will have the number of the first gift.
    /// Consequently, the first gift must not have been withdrawn to the TON blockchain as an NFT and must have an empty gift_address
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn craft_gift(received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::CraftGiftResult, crate::types::Error> {
    let request = json!({
        "@type": "craftGift",
        "received_gift_ids": received_gift_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
