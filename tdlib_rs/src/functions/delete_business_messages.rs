use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes messages on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection through which the messages were received
/// * `message_ids` - Identifier of the messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_messages(
    business_connection_id: String,
    message_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteBusinessMessages",
    "business_connection_id": business_connection_id,
    "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
