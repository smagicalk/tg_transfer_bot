use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a default placeholder for Web Apps of a bot. This is an offline method. Returns a 404 error if the placeholder isn't known
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_placeholder(
    bot_user_id: i64,
    client_id: i32,
) -> Result<crate::enums::Outline, crate::types::Error> {
    let request = json!({
    "@type": "getWebAppPlaceholder",
    "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
