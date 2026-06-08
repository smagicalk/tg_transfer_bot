use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a file by its remote identifier. This is an offline method. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user.
/// For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
/// # Arguments
/// * `remote_file_id` - Remote identifier of the file to get
/// * `file_type` - File type; pass null if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_remote_file(
    remote_file_id: String,
    file_type: Option<crate::enums::FileType>,
    client_id: i32,
) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
    "@type": "getRemoteFile",
    "remote_file_id": remote_file_id,
    "file_type": file_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
