#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. Can be called synchronously
/// # Arguments
/// * `file_name` - The name of the file or path to the file
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_file_mime_type(file_name: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getFileMimeType",
        "file_name": file_name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
