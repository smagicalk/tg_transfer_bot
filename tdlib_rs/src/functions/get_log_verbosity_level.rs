use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns current verbosity level of the internal logging of TDLib. Can be called synchronously
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_log_verbosity_level(
    client_id: i32,
) -> Result<crate::enums::LogVerbosityLevel, crate::types::Error> {
    let request = json!({
    "@type": "getLogVerbosityLevel",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
