#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a URL for upgraded gift withdrawal in the TON blockchain as an NFT; requires owner privileges for gifts owned by a chat
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_withdrawal_url(received_gift_id: String, password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftWithdrawalUrl",
        "received_gift_id": received_gift_id,
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
