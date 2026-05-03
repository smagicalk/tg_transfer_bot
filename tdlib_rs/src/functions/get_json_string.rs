use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Converts a JsonValue object to corresponding JSON-serialized string. Can be called synchronously
/// # Arguments
/// * `json_value` - The JsonValue object
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_json_string(
    json_value: crate::enums::JsonValue,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "getJsonString",
    "json_value": json_value,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
