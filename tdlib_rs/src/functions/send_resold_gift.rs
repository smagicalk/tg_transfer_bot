#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends an upgraded gift that is available for resale to another user or channel chat; gifts already owned by the current user
/// must be transferred using transferGift and can't be passed to the method
/// # Arguments
/// * `gift_name` - Name of the upgraded gift to send
/// * `owner_id` - Identifier of the user or the channel chat that will receive the gift
/// * `price` - The price that the user agreed to pay for the gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_resold_gift(gift_name: String, owner_id: crate::enums::MessageSender, price: crate::enums::GiftResalePrice, client_id: i32) -> Result<crate::enums::GiftResaleResult, crate::types::Error> {
    let request = json!({
        "@type": "sendResoldGift",
        "gift_name": gift_name,
        "owner_id": owner_id,
        "price": price,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
