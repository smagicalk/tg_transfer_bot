#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns all possible variants of upgraded gifts for a regular gift
/// # Arguments
/// * `regular_gift_id` - Identifier of the regular gift
/// * `return_upgrade_models` - Pass true to get models that can be obtained by upgrading a regular gift
/// * `return_craft_models` - Pass true to get models that can be obtained by crafting a gift from upgraded gifts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_variants(regular_gift_id: i64, return_upgrade_models: bool, return_craft_models: bool, client_id: i32) -> Result<crate::enums::GiftUpgradeVariants, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftVariants",
        "regular_gift_id": regular_gift_id,
        "return_upgrade_models": return_upgrade_models,
        "return_craft_models": return_craft_models,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
