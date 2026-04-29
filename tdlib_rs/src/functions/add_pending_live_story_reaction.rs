#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds pending paid reaction in a live story group call. Can't be used in live stories posted by the current user.
/// Call commitPendingLiveStoryReactions or removePendingLiveStoryReactions to actually send all pending reactions when the undo timer is over or abort the sending
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `star_count` - Number of Telegram Stars to be used for the reaction. The total number of pending paid reactions must not exceed getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_pending_live_story_reaction(group_call_id: i32, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addPendingLiveStoryReaction",
        "group_call_id": group_call_id,
        "star_count": star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
