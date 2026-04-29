#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the user opened the sponsored chat via the button, the name, the chat photo, a mention in the sponsored message text, or the media in the sponsored message
/// # Arguments
/// * `chat_id` - Chat identifier of the sponsored message
/// * `message_id` - Identifier of the sponsored message
/// * `is_media_click` - Pass true if the media was clicked in the sponsored message
/// * `from_fullscreen` - Pass true if the user expanded the video from the sponsored message fullscreen before the click
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn click_chat_sponsored_message(chat_id: i64, message_id: i64, is_media_click: bool, from_fullscreen: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickChatSponsoredMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_media_click": is_media_click,
        "from_fullscreen": from_fullscreen,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
