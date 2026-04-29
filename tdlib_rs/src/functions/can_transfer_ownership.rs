#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks whether the current session can be used to transfer a chat ownership to another user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_transfer_ownership(client_id: i32) -> Result<crate::enums::CanTransferOwnershipResult, crate::types::Error> {
    let request = json!({
        "@type": "canTransferOwnership",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
