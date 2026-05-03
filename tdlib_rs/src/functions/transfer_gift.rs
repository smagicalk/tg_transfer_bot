use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends an upgraded gift to another user or channel chat
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `new_owner_id` - Identifier of the user or the channel chat that will receive the gift
/// * `star_count` - The Telegram Star amount required to pay for the transfer
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn transfer_gift(
    received_gift_id: String,
    new_owner_id: crate::enums::MessageSender,
    star_count: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "transferGift",
    "received_gift_id": received_gift_id,
    "new_owner_id": new_owner_id,
    "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
