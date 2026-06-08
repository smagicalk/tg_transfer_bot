use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the current weather in the given location
/// # Arguments
/// * `location` - The location
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_current_weather(
    location: crate::types::Location,
    client_id: i32,
) -> Result<crate::enums::CurrentWeather, crate::types::Error> {
    let request = json!({
    "@type": "getCurrentWeather",
    "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
