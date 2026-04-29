#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the number of times the supergroup must be boosted by a user to ignore slow mode and chat permission restrictions; requires can_restrict_members administrator right
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `unrestrict_boost_count` - New value of the unrestrict_boost_count supergroup setting; 0-8. Use 0 to remove the setting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_unrestrict_boost_count(supergroup_id: i64, unrestrict_boost_count: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupUnrestrictBoostCount",
        "supergroup_id": supergroup_id,
        "unrestrict_boost_count": unrestrict_boost_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
