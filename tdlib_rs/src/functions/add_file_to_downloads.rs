#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a file from a message to the list of file downloads. Download progress and completion of the download will be notified through updateFile updates.
/// If message database is used, the list of file downloads is persistent across application restarts. The downloading is independent of download using downloadFile, i.e. it continues if downloadFile is canceled or is used to download a part of the file
/// # Arguments
/// * `file_id` - Identifier of the file to download
/// * `chat_id` - Chat identifier of the message with the file
/// * `message_id` - Message identifier
/// * `priority` - Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_file_to_downloads(file_id: i32, chat_id: i64, message_id: i64, priority: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "addFileToDownloads",
        "file_id": file_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "priority": priority,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
