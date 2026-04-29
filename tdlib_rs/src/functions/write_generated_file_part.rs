#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Writes a part of a generated file. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
/// # Arguments
/// * `generation_id` - The identifier of the generation process
/// * `offset` - The offset from which to write the data to the file
/// * `data` - The data to write
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn write_generated_file_part(generation_id: i64, offset: i64, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "writeGeneratedFilePart",
        "generation_id": generation_id,
        "offset": offset,
        "data": data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
