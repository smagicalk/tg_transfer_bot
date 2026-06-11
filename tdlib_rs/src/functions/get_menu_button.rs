use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns menu button set by the bot for the given user; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user or 0 to get the default menu button
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_menu_button(
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::BotMenuButton, crate::types::Error> {
    let request = json!({
    "@type": "getMenuButton",
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
