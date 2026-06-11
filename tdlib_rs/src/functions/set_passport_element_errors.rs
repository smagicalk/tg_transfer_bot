use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs the user who some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
/// # Arguments
/// * `user_id` - User identifier
/// * `errors` - The errors
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_passport_element_errors(
    user_id: i64,
    errors: Vec<crate::types::InputPassportElementError>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setPassportElementErrors",
    "user_id": user_id,
    "errors": errors,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
