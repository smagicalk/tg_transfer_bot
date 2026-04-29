#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the verbosity level of the internal logging of TDLib. Can be called synchronously
/// # Arguments
/// * `new_verbosity_level` - New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings,
    /// value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_log_verbosity_level(new_verbosity_level: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLogVerbosityLevel",
        "new_verbosity_level": new_verbosity_level,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
