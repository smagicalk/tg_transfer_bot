use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets support information for the given user; for Telegram support only
/// # Arguments
/// * `user_id` - User identifier
/// * `message` - New information message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_user_support_info(
    user_id: i64,
    message: crate::types::FormattedText,
    client_id: i32,
) -> Result<crate::enums::UserSupportInfo, crate::types::Error> {
    let request = json!({
    "@type": "setUserSupportInfo",
    "user_id": user_id,
    "message": message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
