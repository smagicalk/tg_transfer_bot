#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib on a file generation progress
/// # Arguments
/// * `generation_id` - The identifier of the generation process
/// * `expected_size` - Expected size of the generated file, in bytes; 0 if unknown
/// * `local_prefix_size` - The number of bytes already generated
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_file_generation_progress(generation_id: i64, expected_size: i64, local_prefix_size: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setFileGenerationProgress",
        "generation_id": generation_id,
        "expected_size": expected_size,
        "local_prefix_size": local_prefix_size,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
