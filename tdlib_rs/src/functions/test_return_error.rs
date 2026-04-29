#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the specified error and ensures that the Error object is used; for testing only. Can be called synchronously
/// # Arguments
/// * `error` - The error to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_return_error(error: crate::types::Error, client_id: i32) -> Result<crate::enums::Error, crate::types::Error> {
    let request = json!({
        "@type": "testReturnError",
        "error": error,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
