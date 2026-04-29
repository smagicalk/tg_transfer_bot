#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
/// # Arguments
/// * `file_id` - Identifier of the file. The file must be located in the TDLib file cache
/// * `offset` - The offset from which to read the file
/// * `count` - Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn read_file_part(file_id: i32, offset: i64, count: i64, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "readFilePart",
        "file_id": file_id,
        "offset": offset,
        "count": count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
