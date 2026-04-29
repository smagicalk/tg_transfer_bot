#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns current verbosity level for a specified TDLib internal log tag. Can be called synchronously
/// # Arguments
/// * `tag` - Logging tag to change verbosity level
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_log_tag_verbosity_level(tag: String, client_id: i32) -> Result<crate::enums::LogVerbosityLevel, crate::types::Error> {
    let request = json!({
        "@type": "getLogTagVerbosityLevel",
        "tag": tag,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
