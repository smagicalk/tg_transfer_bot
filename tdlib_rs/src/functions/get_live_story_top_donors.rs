use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of top live story donors
/// # Arguments
/// * `group_call_id` - Group call identifier of the live story
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_top_donors(
    group_call_id: i32,
    client_id: i32,
) -> Result<crate::enums::LiveStoryDonors, crate::types::Error> {
    let request = json!({
    "@type": "getLiveStoryTopDonors",
    "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
