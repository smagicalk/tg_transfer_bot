#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns file downloaded prefix size from a given offset, in bytes
/// # Arguments
/// * `file_id` - Identifier of the file
/// * `offset` - Offset from which downloaded prefix size needs to be calculated
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_file_downloaded_prefix_size(file_id: i32, offset: i64, client_id: i32) -> Result<crate::enums::FileDownloadedPrefixSize, crate::types::Error> {
    let request = json!({
        "@type": "getFileDownloadedPrefixSize",
        "file_id": file_id,
        "offset": offset,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
