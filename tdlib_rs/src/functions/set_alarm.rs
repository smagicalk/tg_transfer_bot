use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Succeeds after a specified amount of time has passed. Can be called before initialization
/// # Arguments
/// * `seconds` - Number of seconds before the function returns
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_alarm(seconds: f64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAlarm",
    "seconds": seconds,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
