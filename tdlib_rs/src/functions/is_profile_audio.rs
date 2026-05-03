use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether a file is in the profile audio files of the current user. Returns a 404 error if it isn't
/// # Arguments
/// * `file_id` - Identifier of the audio file to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn is_profile_audio(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "isProfileAudio",
    "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
