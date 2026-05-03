use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes all call messages
/// # Arguments
/// * `revoke` - Pass true to delete the messages for all users
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_call_messages(
    revoke: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteAllCallMessages",
    "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
