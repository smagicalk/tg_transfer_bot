use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns all contacts of the user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_contacts(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
    "@type": "getContacts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
