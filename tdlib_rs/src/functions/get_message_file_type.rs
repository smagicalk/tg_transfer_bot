#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a file with messages exported from another application
/// # Arguments
/// * `message_file_head` - Beginning of the message file; up to 100 first lines
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_file_type(message_file_head: String, client_id: i32) -> Result<crate::enums::MessageFileType, crate::types::Error> {
    let request = json!({
        "@type": "getMessageFileType",
        "message_file_head": message_file_head,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
