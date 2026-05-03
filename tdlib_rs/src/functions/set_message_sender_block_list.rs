use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the block list of a message sender. Currently, only users and supergroup chats can be blocked
/// # Arguments
/// * `sender_id` - Identifier of a message sender to block/unblock
/// * `block_list` - New block list for the message sender; pass null to unblock the message sender
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_message_sender_block_list(
    sender_id: crate::enums::MessageSender,
    block_list: Option<crate::enums::BlockList>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setMessageSenderBlockList",
    "sender_id": sender_id,
    "block_list": block_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
