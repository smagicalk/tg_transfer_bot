use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a Web App by its short name. Returns a 404 error if the Web App is not found
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `web_app_short_name` - Short name of the Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_web_app(
    bot_user_id: i64,
    web_app_short_name: String,
    client_id: i32,
) -> Result<crate::enums::FoundWebApp, crate::types::Error> {
    let request = json!({
    "@type": "searchWebApp",
    "bot_user_id": bot_user_id,
    "web_app_short_name": web_app_short_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
