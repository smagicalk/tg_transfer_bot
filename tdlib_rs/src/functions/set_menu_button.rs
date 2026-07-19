use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets menu button for the given user or for all users; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user or 0 to set menu button for all users
/// * `menu_button` - New menu button
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_menu_button(
    user_id: i64,
    menu_button: crate::types::BotMenuButton,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setMenuButton",
    "user_id": user_id,
    "menu_button": menu_button,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
