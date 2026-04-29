#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a file. This is an offline method
/// # Arguments
/// * `file_id` - Identifier of the file to get
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_file(file_id: i32, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "getFile",
        "file_id": file_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
