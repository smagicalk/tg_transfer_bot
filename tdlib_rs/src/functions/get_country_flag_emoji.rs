use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an emoji for the given country. Returns an empty string on failure. Can be called synchronously
/// # Arguments
/// * `country_code` - A two-letter ISO 3166-1 alpha-2 country code as received from getCountries
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_country_flag_emoji(
    country_code: String,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "getCountryFlagEmoji",
    "country_code": country_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
