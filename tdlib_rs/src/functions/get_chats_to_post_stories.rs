#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns supergroup and channel chats in which the current user has the right to post stories. The chats must be rechecked with canPostStory before actually trying to post a story there
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chats_to_post_stories(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatsToPostStories",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
