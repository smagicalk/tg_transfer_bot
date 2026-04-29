#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes all pending paid reactions in a live story group call
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_pending_live_story_reactions(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removePendingLiveStoryReactions",
        "group_call_id": group_call_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
