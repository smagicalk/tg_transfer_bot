use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a business connection by its identifier; for bots only
/// # Arguments
/// * `connection_id` - Identifier of the business connection to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_business_connection(
    connection_id: String,
    client_id: i32,
) -> Result<crate::enums::BusinessConnection, crate::types::Error> {
    let request = json!({
    "@type": "getBusinessConnection",
    "connection_id": connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
