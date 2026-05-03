use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a passkey allowed to be used for the login by the current user and returns the added passkey. Call getPasskeyParameters to get parameters for creating of the passkey
/// # Arguments
/// * `client_data` - JSON-encoded client data
/// * `attestation_object` - Passkey attestation object
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_login_passkey(
    client_data: String,
    attestation_object: String,
    client_id: i32,
) -> Result<crate::enums::Passkey, crate::types::Error> {
    let request = json!({
    "@type": "addLoginPasskey",
    "client_data": client_data,
    "attestation_object": attestation_object,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
