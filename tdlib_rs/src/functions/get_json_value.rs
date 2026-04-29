#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
/// # Arguments
/// * `json` - The JSON-serialized string
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_json_value(json: String, client_id: i32) -> Result<crate::enums::JsonValue, crate::types::Error> {
    let request = json!({
        "@type": "getJsonValue",
        "json": json,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
