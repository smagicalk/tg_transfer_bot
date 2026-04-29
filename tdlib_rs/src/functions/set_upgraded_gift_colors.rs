#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes color scheme for the current user based on an owned or a hosted upgraded gift; for Telegram Premium users only
/// # Arguments
/// * `upgraded_gift_colors_id` - Identifier of the upgradedGiftColors scheme to use
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_upgraded_gift_colors(upgraded_gift_colors_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUpgradedGiftColors",
        "upgraded_gift_colors_id": upgraded_gift_colors_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
