use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Handles all pending join requests for a given link in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to process join requests. If empty, all join requests will be processed. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// * `approve` - Pass true to approve all requests; pass false to decline them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_join_requests(
    chat_id: i64,
    invite_link: String,
    approve: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "processChatJoinRequests",
    "chat_id": chat_id,
    "invite_link": invite_link,
    "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
