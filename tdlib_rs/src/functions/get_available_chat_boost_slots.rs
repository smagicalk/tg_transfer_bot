#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of available chat boost slots for the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_available_chat_boost_slots(client_id: i32) -> Result<crate::enums::ChatBoostSlots, crate::types::Error> {
    let request = json!({
        "@type": "getAvailableChatBoostSlots",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
