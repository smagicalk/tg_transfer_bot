#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a file from the file download list
/// # Arguments
/// * `file_id` - Identifier of the downloaded file
/// * `delete_from_cache` - Pass true to delete the file from the TDLib file cache
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_file_from_downloads(file_id: i32, delete_from_cache: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeFileFromDownloads",
        "file_id": file_id,
        "delete_from_cache": delete_from_cache,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
