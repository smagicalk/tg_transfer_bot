#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the chat theme. Supported only in private and secret chats
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `theme` - New chat theme; pass null to return the default theme
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_theme(chat_id: i64, theme: Option<crate::enums::InputChatTheme>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatTheme",
        "chat_id": chat_id,
        "theme": theme,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
