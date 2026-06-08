use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of profile audio files of a user
/// # Arguments
/// * `user_id` - User identifier
/// * `offset` - The number of audio files to skip; must be non-negative
/// * `limit` - The maximum number of audio files to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_profile_audios(
    user_id: i64,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Audios, crate::types::Error> {
    let request = json!({
    "@type": "getUserProfileAudios",
    "user_id": user_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
