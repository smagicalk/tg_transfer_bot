#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. Can be called synchronously
/// # Arguments
/// * `mime_type` - The MIME type of the file
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_file_extension(mime_type: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getFileExtension",
        "mime_type": mime_type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
