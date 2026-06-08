use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Finishes the file generation
/// # Arguments
/// * `generation_id` - The identifier of the generation process
/// * `error` - If passed, the file generation has failed and must be terminated; pass null if the file generation succeeded
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn finish_file_generation(
    generation_id: i64,
    error: Option<crate::types::Error>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "finishFileGeneration",
    "generation_id": generation_id,
    "error": error,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
