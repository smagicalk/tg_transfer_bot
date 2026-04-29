#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Pays for upgrade of a regular gift that is owned by another user or channel chat
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the gift
/// * `prepaid_upgrade_hash` - Prepaid upgrade hash as received along with the gift
/// * `star_count` - The Telegram Star amount the user agreed to pay for the upgrade; must be equal to gift.upgrade_star_count
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn buy_gift_upgrade(owner_id: crate::enums::MessageSender, prepaid_upgrade_hash: String, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "buyGiftUpgrade",
        "owner_id": owner_id,
        "prepaid_upgrade_hash": prepaid_upgrade_hash,
        "star_count": star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
