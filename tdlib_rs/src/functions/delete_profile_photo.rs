use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes a profile photo
/// # Arguments
/// * `profile_photo_id` - Identifier of the profile photo to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_profile_photo(
    profile_photo_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteProfilePhoto",
    "profile_photo_id": profile_photo_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
