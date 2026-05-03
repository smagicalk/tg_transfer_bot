use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether joining is mandatory to send messages to a discussion supergroup; requires can_restrict_members administrator right
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup that isn't a broadcast group
/// * `join_to_send_messages` - New value of join_to_send_messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_join_to_send_messages(
    supergroup_id: i64,
    join_to_send_messages: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleSupergroupJoinToSendMessages",
    "supergroup_id": supergroup_id,
    "join_to_send_messages": join_to_send_messages,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
