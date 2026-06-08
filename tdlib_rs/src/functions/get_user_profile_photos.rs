use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the profile photos of a user. Personal and public photo aren't returned
/// # Arguments
/// * `user_id` - User identifier
/// * `offset` - The number of photos to skip; must be non-negative
/// * `limit` - The maximum number of photos to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_profile_photos(
    user_id: i64,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ChatPhotos, crate::types::Error> {
    let request = json!({
    "@type": "getUserProfilePhotos",
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
