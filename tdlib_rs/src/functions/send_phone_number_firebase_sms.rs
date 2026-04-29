#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends Firebase Authentication SMS to the specified phone number. Works only when received a code of the type authenticationCodeTypeFirebaseAndroid or authenticationCodeTypeFirebaseIos
/// # Arguments
/// * `token` - Play Integrity API or SafetyNet Attestation API token for the Android application, or secret from push notification for the iOS application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_phone_number_firebase_sms(token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendPhoneNumberFirebaseSms",
        "token": token,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
