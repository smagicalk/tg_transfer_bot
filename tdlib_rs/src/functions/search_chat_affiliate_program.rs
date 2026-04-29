#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches a chat with an affiliate program. Returns the chat if found and the program is active
/// # Arguments
/// * `username` - Username of the chat
/// * `referrer` - The referrer from an internalLinkTypeChatAffiliateProgram link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_affiliate_program(username: String, referrer: String, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "searchChatAffiliateProgram",
        "username": username,
        "referrer": referrer,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
