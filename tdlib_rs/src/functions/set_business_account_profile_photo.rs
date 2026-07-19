use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes a profile photo of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `photo` - Profile photo to set; pass null to remove the photo
/// * `is_public` - Pass true to set the public photo, which will be visible even if the main photo is hidden by privacy settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_profile_photo(
    business_connection_id: String,
    photo: Option<crate::enums::InputChatPhoto>,
    is_public: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessAccountProfilePhoto",
    "business_connection_id": business_connection_id,
    "photo": photo,
    "is_public": is_public,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
