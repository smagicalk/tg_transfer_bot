use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first.
/// Only non-secret video animations with MIME type "video/mp4" can be added to the list
/// # Arguments
/// * `animation` - The animation file to be added. Only animations known to the server (i.e., successfully sent via a message) can be added to the list
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_saved_animation(
    animation: crate::enums::InputFile,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addSavedAnimation",
    "animation": animation,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
