use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about members or banned users in a supergroup or channel. Can be used only if supergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `filter` - The type of users to return; pass null to use supergroupMembersFilterRecent
/// * `offset` - Number of users to skip
/// * `limit` - The maximum number of users to be returned; up to 200
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_supergroup_members(
    supergroup_id: i64,
    filter: Option<crate::enums::SupergroupMembersFilter>,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ChatMembers, crate::types::Error> {
    let request = json!({
    "@type": "getSupergroupMembers",
    "supergroup_id": supergroup_id,
    "filter": filter,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
