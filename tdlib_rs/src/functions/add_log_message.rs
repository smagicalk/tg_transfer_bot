use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a message to TDLib internal log. Can be called synchronously
/// # Arguments
/// * `verbosity_level` - The minimum verbosity level needed for the message to be logged; 0-1023
/// * `text` - Text of a message to log
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_log_message(
    verbosity_level: i32,
    text: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addLogMessage",
    "verbosity_level": verbosity_level,
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
