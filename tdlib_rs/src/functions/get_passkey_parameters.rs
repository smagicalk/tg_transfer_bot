#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns parameters for creating of a new passkey as JSON-serialized string
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_passkey_parameters(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getPasskeyParameters",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
