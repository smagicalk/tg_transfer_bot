use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Uses the current IP address to find the current country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_country_code(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "getCountryCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
