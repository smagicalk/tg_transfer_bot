use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes position of an audio file in the profile audio files of the current user
/// # Arguments
/// * `file_id` - Identifier of the file from profile audio files, which position will be changed
/// * `after_file_id` - Identifier of the file from profile audio files after which the file will be positioned; pass 0 to move the file to the beginning of the list
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_profile_audio_position(
    file_id: i32,
    after_file_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setProfileAudioPosition",
    "file_id": file_id,
    "after_file_id": after_file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
