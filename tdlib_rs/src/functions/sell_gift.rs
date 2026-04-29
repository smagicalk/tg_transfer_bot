#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sells a gift for Telegram Stars; requires owner privileges for gifts owned by a chat
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn sell_gift(received_gift_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sellGift",
        "received_gift_id": received_gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
