use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information needed to open the main Web App of a bot
/// # Arguments
/// * `chat_id` - Identifier of the chat in which the Web App is opened; pass 0 if none
/// * `bot_user_id` - Identifier of the target bot. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `start_parameter` - Start parameter from internalLinkTypeMainWebApp
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_main_web_app(
    chat_id: i64,
    bot_user_id: i64,
    start_parameter: String,
    parameters: crate::types::WebAppOpenParameters,
    client_id: i32,
) -> Result<crate::enums::MainWebApp, crate::types::Error> {
    let request = json!({
    "@type": "getMainWebApp",
    "chat_id": chat_id,
    "bot_user_id": bot_user_id,
    "start_parameter": start_parameter,
    "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
