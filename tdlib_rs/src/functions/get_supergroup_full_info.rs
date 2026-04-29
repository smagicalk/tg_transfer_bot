#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
/// # Arguments
/// * `supergroup_id` - Supergroup or channel identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_supergroup_full_info(supergroup_id: i64, client_id: i32) -> Result<crate::enums::SupergroupFullInfo, crate::types::Error> {
    let request = json!({
        "@type": "getSupergroupFullInfo",
        "supergroup_id": supergroup_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
