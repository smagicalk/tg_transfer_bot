#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether sender signature or link to the account is added to sent messages in a channel; requires can_change_info member right
/// # Arguments
/// * `supergroup_id` - Identifier of the channel
/// * `sign_messages` - New value of sign_messages
/// * `show_message_sender` - New value of show_message_sender
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_sign_messages(supergroup_id: i64, sign_messages: bool, show_message_sender: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupSignMessages",
        "supergroup_id": supergroup_id,
        "sign_messages": sign_messages,
        "show_message_sender": show_message_sender,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
