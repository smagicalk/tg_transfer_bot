#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Drops original details for an upgraded gift
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `star_count` - The Telegram Star amount required to pay for the operation
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn drop_gift_original_details(received_gift_id: String, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "dropGiftOriginalDetails",
        "received_gift_id": received_gift_id,
        "star_count": star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
