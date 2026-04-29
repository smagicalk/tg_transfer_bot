#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
/// # Arguments
/// * `file_id` - Identifier of the file to download
/// * `priority` - Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first
/// * `offset` - The starting position from which the file needs to be downloaded
/// * `limit` - Number of bytes which need to be downloaded starting from the "offset" position before the download will automatically be canceled; use 0 to download without a limit
/// * `synchronous` - Pass true to return response only after the file download has succeeded, has failed, has been canceled, or a new downloadFile request with different offset/limit parameters was sent; pass false to return file state immediately, just after the download has been started
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn download_file(file_id: i32, priority: i32, offset: i64, limit: i64, synchronous: bool, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "downloadFile",
        "file_id": file_id,
        "priority": priority,
        "offset": offset,
        "limit": limit,
        "synchronous": synchronous,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
