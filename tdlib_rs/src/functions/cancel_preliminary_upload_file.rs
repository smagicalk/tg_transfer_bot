use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Stops the preliminary uploading of a file. Supported only for files uploaded by using preliminaryUploadFile
/// # Arguments
/// * `file_id` - Identifier of the file to stop uploading
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn cancel_preliminary_upload_file(
    file_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "cancelPreliminaryUploadFile",
    "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
