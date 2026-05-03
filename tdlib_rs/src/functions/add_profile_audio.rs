use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds an audio file to the beginning of the profile audio files of the current user
/// # Arguments
/// * `file_id` - Identifier of the audio file to be added. The file must have been uploaded to the server
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_profile_audio(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addProfileAudio",
    "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
