use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes privacy settings for new chat creation; can be used only if getOption("can_set_new_chat_privacy_settings")
/// # Arguments
/// * `settings` - New settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_new_chat_privacy_settings(
    settings: crate::types::NewChatPrivacySettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setNewChatPrivacySettings",
    "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
