use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the received string; for testing only. This is an offline method. Can be called before authorization
/// # Arguments
/// * `x` - String to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_call_string(
    x: String,
    client_id: i32,
) -> Result<crate::enums::TestString, crate::types::Error> {
    let request = json!({
    "@type": "testCallString",
    "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
