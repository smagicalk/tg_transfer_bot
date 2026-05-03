use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a phone number by its prefix synchronously. getCountries must be called at least once after changing localization to the specified language if properly localized country information is expected. Can be called synchronously
/// # Arguments
/// * `language_code` - A two-letter ISO 639-1 language code for country information localization
/// * `phone_number_prefix` - The phone number prefix
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_phone_number_info_sync(
    language_code: String,
    phone_number_prefix: String,
    client_id: i32,
) -> Result<crate::enums::PhoneNumberInfo, crate::types::Error> {
    let request = json!({
    "@type": "getPhoneNumberInfoSync",
    "language_code": language_code,
    "phone_number_prefix": phone_number_prefix,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
