use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info member right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `photo` - New chat photo; pass null to delete the chat photo
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_photo(
    chat_id: i64,
    photo: Option<crate::enums::InputChatPhoto>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatPhoto",
    "chat_id": chat_id,
    "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
