use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Saves an inline message to be sent by the given user; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user
/// * `result` - The description of the message
/// * `chat_types` - Types of the chats to which the message can be sent
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn save_prepared_inline_message(
    user_id: i64,
    result: crate::enums::InputInlineQueryResult,
    chat_types: crate::types::TargetChatTypes,
    client_id: i32,
) -> Result<crate::enums::PreparedInlineMessageId, crate::types::Error> {
    let request = json!({
    "@type": "savePreparedInlineMessage",
    "user_id": user_id,
    "result": result,
    "chat_types": chat_types,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
