use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes potentially dangerous characters from the name of a file. Returns an empty string on failure. Can be called synchronously
/// # Arguments
/// * `file_name` - File name or path to the file
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clean_file_name(
    file_name: String,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "cleanFileName",
    "file_name": file_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
