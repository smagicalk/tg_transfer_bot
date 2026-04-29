#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns suggested name for saving a file in a given directory
/// # Arguments
/// * `file_id` - Identifier of the file
/// * `directory` - Directory in which the file is expected to be saved
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_suggested_file_name(file_id: i32, directory: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getSuggestedFileName",
        "file_id": file_id,
        "directory": directory,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
