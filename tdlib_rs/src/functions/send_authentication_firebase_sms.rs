use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends Firebase Authentication SMS to the phone number of the user. Works only when the current authorization state is authorizationStateWaitCode and the server returned code of the type authenticationCodeTypeFirebaseAndroid or authenticationCodeTypeFirebaseIos
/// # Arguments
/// * `token` - Play Integrity API or SafetyNet Attestation API token for the Android application, or secret from push notification for the iOS application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_authentication_firebase_sms(
    token: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "sendAuthenticationFirebaseSms",
    "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
