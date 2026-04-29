#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Preliminarily uploads a file to the cloud before sending it in a message, which can be useful for uploading of being recorded voice and video notes.
/// In all other cases there is no need to preliminary upload a file. Updates updateFile will be used to notify about upload progress.
/// The upload will not be completed until the file is sent in a message
/// # Arguments
/// * `file` - File to upload
/// * `file_type` - File type; pass null if unknown
/// * `priority` - Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which preliminaryUploadFile was called will be uploaded first
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn preliminary_upload_file(file: crate::enums::InputFile, file_type: Option<crate::enums::FileType>, priority: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "preliminaryUploadFile",
        "file": file,
        "file_type": file_type,
        "priority": priority,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
