#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a received gift
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_received_gift(received_gift_id: String, client_id: i32) -> Result<crate::enums::ReceivedGift, crate::types::Error> {
    let request = json!({
        "@type": "getReceivedGift",
        "received_gift_id": received_gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
