use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Suggests a profile photo to another regular user with common messages and allowing non-paid messages
/// # Arguments
/// * `user_id` - User identifier
/// * `photo` - Profile photo to suggest; inputChatPhotoPrevious isn't supported in this function
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn suggest_user_profile_photo(
    user_id: i64,
    photo: crate::enums::InputChatPhoto,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "suggestUserProfilePhoto",
    "user_id": user_id,
    "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
