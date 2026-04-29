#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns pending join requests in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to return join requests. If empty, all join requests will be returned. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// * `query` - A query to search for in the first names, last names and usernames of the users to return
/// * `offset_request` - A chat join request from which to return next requests; pass null to get results from the beginning
/// * `limit` - The maximum number of requests to join the chat to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_join_requests(chat_id: i64, invite_link: String, query: String, offset_request: Option<crate::types::ChatJoinRequest>, limit: i32, client_id: i32) -> Result<crate::enums::ChatJoinRequests, crate::types::Error> {
    let request = json!({
        "@type": "getChatJoinRequests",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "query": query,
        "offset_request": offset_request,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
