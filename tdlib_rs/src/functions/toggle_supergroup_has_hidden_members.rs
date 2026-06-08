use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers. Can be called only if supergroupFullInfo.can_hide_members == true
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `has_hidden_members` - New value of has_hidden_members
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_hidden_members(
    supergroup_id: i64,
    has_hidden_members: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleSupergroupHasHiddenMembers",
    "supergroup_id": supergroup_id,
    "has_hidden_members": has_hidden_members,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
