#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
/// # Arguments
/// * `x` - Vector of objects to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_call_vector_string_object(x: Vec<crate::types::TestString>, client_id: i32) -> Result<crate::enums::TestVectorStringObject, crate::types::Error> {
    let request = json!({
        "@type": "testCallVectorStringObject",
        "x": x,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
