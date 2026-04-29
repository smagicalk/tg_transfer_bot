#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether the current user can message another user or try to create a chat with them
/// # Arguments
/// * `user_id` - Identifier of the other user
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_send_message_to_user(user_id: i64, only_local: bool, client_id: i32) -> Result<crate::enums::CanSendMessageToUserResult, crate::types::Error> {
    let request = json!({
        "@type": "canSendMessageToUser",
        "user_id": user_id,
        "only_local": only_local,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
