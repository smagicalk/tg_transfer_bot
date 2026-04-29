#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a passkey from the list of passkeys allowed to be used for the login by the current user
/// # Arguments
/// * `passkey_id` - Unique identifier of the passkey to remove
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_login_passkey(passkey_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeLoginPasskey",
        "passkey_id": passkey_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
