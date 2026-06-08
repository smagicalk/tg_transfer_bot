use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns parameters for authentication using a passkey as JSON-serialized string
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_authentication_passkey_parameters(
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "getAuthenticationPasskeyParameters",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
