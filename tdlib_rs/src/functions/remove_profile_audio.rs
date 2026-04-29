#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes an audio file from the profile audio files of the current user
/// # Arguments
/// * `file_id` - Identifier of the audio file to be removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_profile_audio(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeProfileAudio",
        "file_id": file_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
