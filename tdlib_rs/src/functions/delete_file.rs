use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes a file from the TDLib file cache
/// # Arguments
/// * `file_id` - Identifier of the file to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_file(file_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteFile",
    "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
