use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl.
/// Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
/// # Arguments
/// * `chat_id` - Chat identifier of the message with the button
/// * `message_id` - Message identifier of the message with the button
/// * `button_id` - Button identifier
/// * `allow_write_access` - Pass true to allow the bot to send messages to the current user. Phone number access can't be requested using the button
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_login_url(
    chat_id: i64,
    message_id: i64,
    button_id: i64,
    allow_write_access: bool,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getLoginUrl",
    "chat_id": chat_id,
    "message_id": message_id,
    "button_id": button_id,
    "allow_write_access": allow_write_access,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
