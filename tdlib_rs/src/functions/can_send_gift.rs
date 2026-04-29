#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether a gift with next_send_date in the future can be sent already
/// # Arguments
/// * `gift_id` - Identifier of the gift to send
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_send_gift(gift_id: i64, client_id: i32) -> Result<crate::enums::CanSendGiftResult, crate::types::Error> {
    let request = json!({
        "@type": "canSendGift",
        "gift_id": gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
