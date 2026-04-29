#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Clears message drafts in all chats
/// # Arguments
/// * `exclude_secret_chats` - Pass true to keep local message drafts in secret chats
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_all_draft_messages(exclude_secret_chats: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearAllDraftMessages",
        "exclude_secret_chats": exclude_secret_chats,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
