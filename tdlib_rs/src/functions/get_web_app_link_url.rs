use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an HTTPS URL of a Web App to open after a link of the type internalLinkTypeWebApp is clicked
/// # Arguments
/// * `chat_id` - Identifier of the chat in which the link was clicked; pass 0 if none
/// * `bot_user_id` - Identifier of the target bot
/// * `web_app_short_name` - Short name of the Web App
/// * `start_parameter` - Start parameter from internalLinkTypeWebApp
/// * `allow_write_access` - Pass true if the current user allowed the bot to send them messages
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_link_url(
    chat_id: i64,
    bot_user_id: i64,
    web_app_short_name: String,
    start_parameter: String,
    allow_write_access: bool,
    parameters: crate::types::WebAppOpenParameters,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getWebAppLinkUrl",
    "chat_id": chat_id,
    "bot_user_id": bot_user_id,
    "web_app_short_name": web_app_short_name,
    "start_parameter": start_parameter,
    "allow_write_access": allow_write_access,
    "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
