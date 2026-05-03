use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes pause state of all files in the file download list
/// # Arguments
/// * `are_paused` - Pass true to pause all downloads; pass false to unpause them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_all_downloads_are_paused(
    are_paused: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleAllDownloadsArePaused",
    "are_paused": are_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
