use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the editable username of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `username` - The new value of the username
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_username(
    business_connection_id: String,
    username: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessAccountUsername",
    "business_connection_id": business_connection_id,
    "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
