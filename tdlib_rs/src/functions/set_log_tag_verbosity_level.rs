use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the verbosity level for a specified TDLib internal log tag. Can be called synchronously
/// # Arguments
/// * `tag` - Logging tag to change verbosity level
/// * `new_verbosity_level` - New verbosity level; 1-1024
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_log_tag_verbosity_level(
    tag: String,
    new_verbosity_level: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setLogTagVerbosityLevel",
    "tag": tag,
    "new_verbosity_level": new_verbosity_level,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
