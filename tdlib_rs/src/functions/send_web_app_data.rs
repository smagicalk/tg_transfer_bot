#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends data received from a keyboardButtonTypeWebApp Web App to a bot
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `button_text` - Text of the keyboardButtonTypeWebApp button, which opened the Web App
/// * `data` - The data
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_web_app_data(bot_user_id: i64, button_text: String, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendWebAppData",
        "bot_user_id": bot_user_id,
        "button_text": button_text,
        "data": data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
