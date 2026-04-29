#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches a public chat by its username. Currently, only private chats, supergroups and channels can be public. Returns the chat if found; otherwise, an error is returned
/// # Arguments
/// * `username` - Username to be resolved
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_public_chat(username: String, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicChat",
        "username": username,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
