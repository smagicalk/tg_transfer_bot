#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
/// # Arguments
/// * `chat_id` - Chat identifier of the message with the button
/// * `message_id` - Message identifier of the message with the button. The message must not be scheduled
/// * `button_id` - Button identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_login_url_info(chat_id: i64, message_id: i64, button_id: i64, client_id: i32) -> Result<crate::enums::LoginUrlInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLoginUrlInfo",
        "chat_id": chat_id,
        "message_id": message_id,
        "button_id": button_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
