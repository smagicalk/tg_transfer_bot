#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns examples of possible upgraded gifts for a regular gift
/// # Arguments
/// * `regular_gift_id` - Identifier of the regular gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_upgrade_preview(regular_gift_id: i64, client_id: i32) -> Result<crate::enums::GiftUpgradePreview, crate::types::Error> {
    let request = json!({
        "@type": "getGiftUpgradePreview",
        "regular_gift_id": regular_gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
