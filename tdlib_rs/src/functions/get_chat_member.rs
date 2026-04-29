#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a single member of a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_member(chat_id: i64, member_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::ChatMember, crate::types::Error> {
    let request = json!({
        "@type": "getChatMember",
        "chat_id": chat_id,
        "member_id": member_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
