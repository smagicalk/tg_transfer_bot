#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a given collectible item that was purchased at https:fragment.com
/// # Arguments
/// * `r#type` - Type of the collectible item. The item must be used by a user and must be visible to the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_collectible_item_info(r#type: crate::enums::CollectibleItemType, client_id: i32) -> Result<crate::enums::CollectibleItemInfo, crate::types::Error> {
    let request = json!({
        "@type": "getCollectibleItemInfo",
        "type": r#type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
