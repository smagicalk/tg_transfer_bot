#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets new log stream for internal logging of TDLib. Can be called synchronously
/// # Arguments
/// * `log_stream` - New log stream
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_log_stream(log_stream: crate::enums::LogStream, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLogStream",
        "log_stream": log_stream,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
