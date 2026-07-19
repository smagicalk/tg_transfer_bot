use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the first and last name of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `first_name` - The new value of the first name for the business account; 1-64 characters
/// * `last_name` - The new value of the optional last name for the business account; 0-64 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_name(
    business_connection_id: String,
    first_name: String,
    last_name: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessAccountName",
    "business_connection_id": business_connection_id,
    "first_name": first_name,
    "last_name": last_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
