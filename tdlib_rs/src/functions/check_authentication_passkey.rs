use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks a passkey to log in to the corresponding account. Call getAuthenticationPasskeyParameters to get parameters for the passkey. Works only when the current authorization state is
/// authorizationStateWaitPhoneNumber or authorizationStateWaitOtherDeviceConfirmation, or if there is no pending authentication query and the current authorization state is
/// authorizationStateWaitPremiumPurchase, authorizationStateWaitEmailAddress, authorizationStateWaitEmailCode, authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
/// # Arguments
/// * `credential_id` - Base64url-encoded identifier of the credential
/// * `client_data` - JSON-encoded client data
/// * `authenticator_data` - Authenticator data of the application that created the credential
/// * `signature` - Cryptographic signature of the credential
/// * `user_handle` - User handle of the passkey
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_passkey(
    credential_id: String,
    client_data: String,
    authenticator_data: String,
    signature: String,
    user_handle: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkAuthenticationPasskey",
    "credential_id": credential_id,
    "client_data": client_data,
    "authenticator_data": authenticator_data,
    "signature": signature,
    "user_handle": user_handle,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
