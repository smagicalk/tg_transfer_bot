#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns privacy settings for new chat creation
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_new_chat_privacy_settings(client_id: i32) -> Result<crate::enums::NewChatPrivacySettings, crate::types::Error> {
    let request = json!({
        "@type": "getNewChatPrivacySettings",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
