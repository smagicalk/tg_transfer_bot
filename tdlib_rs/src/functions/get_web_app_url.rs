use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an HTTPS URL of a Web App to open from the side menu, a keyboardButtonTypeWebApp button, or an inlineQueryResultsButtonTypeWebApp button
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `url` - The URL from a keyboardButtonTypeWebApp button, inlineQueryResultsButtonTypeWebApp button, or an empty string when the bot is opened from the side menu
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_url(
    bot_user_id: i64,
    url: String,
    parameters: crate::types::WebAppOpenParameters,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getWebAppUrl",
    "bot_user_id": bot_user_id,
    "url": url,
    "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
