#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for a specified query in the first name, last name and usernames of the members of a specified chat. Requires administrator rights if the chat is a channel
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `query` - Query to search for
/// * `limit` - The maximum number of users to be returned; up to 200
/// * `filter` - The type of users to search for; pass null to search among all chat members
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_members(chat_id: i64, query: String, limit: i32, filter: Option<crate::enums::ChatMembersFilter>, client_id: i32) -> Result<crate::enums::ChatMembers, crate::types::Error> {
    let request = json!({
        "@type": "searchChatMembers",
        "chat_id": chat_id,
        "query": query,
        "limit": limit,
        "filter": filter,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
