use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a custom request from a Web App
/// # Arguments
/// * `bot_user_id` - Identifier of the bot
/// * `method` - The method name
/// * `parameters` - JSON-serialized method parameters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_web_app_custom_request(
    bot_user_id: i64,
    method: String,
    parameters: String,
    client_id: i32,
) -> Result<crate::enums::CustomRequestResult, crate::types::Error> {
    let request = json!({
    "@type": "sendWebAppCustomRequest",
    "bot_user_id": bot_user_id,
    "method": method,
    "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
