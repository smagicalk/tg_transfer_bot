#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about the current temporary password
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_temporary_password_state(client_id: i32) -> Result<crate::enums::TemporaryPasswordState, crate::types::Error> {
    let request = json!({
        "@type": "getTemporaryPasswordState",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
