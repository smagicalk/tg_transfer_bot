#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes a profile photo for a bot
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `photo` - Profile photo to set; pass null to delete the chat photo
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_profile_photo(bot_user_id: i64, photo: Option<crate::enums::InputChatPhoto>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotProfilePhoto",
        "bot_user_id": bot_user_id,
        "photo": photo,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
