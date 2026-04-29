#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes a personal profile photo of a contact user
/// # Arguments
/// * `user_id` - User identifier
/// * `photo` - Profile photo to set; pass null to delete the photo; inputChatPhotoPrevious isn't supported in this function
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_user_personal_profile_photo(user_id: i64, photo: Option<crate::enums::InputChatPhoto>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserPersonalProfilePhoto",
        "user_id": user_id,
        "photo": photo,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
