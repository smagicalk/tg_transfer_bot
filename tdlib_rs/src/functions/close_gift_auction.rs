#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that a gift auction was closed by the user
/// # Arguments
/// * `gift_id` - Identifier of the gift, which auction was closed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn close_gift_auction(gift_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeGiftAuction",
        "gift_id": gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
