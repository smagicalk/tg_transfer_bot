#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes pause state of a file in the file download list
/// # Arguments
/// * `file_id` - Identifier of the downloaded file
/// * `is_paused` - Pass true if the download is paused
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_download_is_paused(file_id: i32, is_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleDownloadIsPaused",
        "file_id": file_id,
        "is_paused": is_paused,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
