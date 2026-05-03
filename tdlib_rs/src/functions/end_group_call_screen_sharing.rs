use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Ends screen sharing in a joined group call; not supported in live stories
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn end_group_call_screen_sharing(
    group_call_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "endGroupCallScreenSharing",
    "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
