#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an IETF language tag of the language preferred in the country, which must be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
/// # Arguments
/// * `country_code` - A two-letter ISO 3166-1 alpha-2 country code
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_preferred_country_language(country_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getPreferredCountryLanguage",
        "country_code": country_code,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
