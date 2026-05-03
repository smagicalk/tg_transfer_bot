use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the translatable state of a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `is_translatable` - New value of is_translatable
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_translatable(
    chat_id: i64,
    is_translatable: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleChatIsTranslatable",
    "chat_id": chat_id,
    "is_translatable": is_translatable,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
