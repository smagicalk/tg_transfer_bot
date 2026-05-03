use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Stops the downloading of a file. If a file has already been downloaded, does nothing
/// # Arguments
/// * `file_id` - Identifier of a file to stop downloading
/// * `only_if_pending` - Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn cancel_download_file(
    file_id: i32,
    only_if_pending: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "cancelDownloadFile",
    "file_id": file_id,
    "only_if_pending": only_if_pending,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
