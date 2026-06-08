use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached. The limit can be increased with Telegram Premium
/// # Arguments
/// * `r#type` - Type of the public chats, for which to check the limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_created_public_chats_limit(
    r#type: crate::enums::PublicChatType,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkCreatedPublicChatsLimit",
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
