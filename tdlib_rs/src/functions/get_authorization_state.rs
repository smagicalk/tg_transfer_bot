#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the current authorization state. This is an offline method. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_authorization_state(client_id: i32) -> Result<crate::enums::AuthorizationState, crate::types::Error> {
    let request = json!({
        "@type": "getAuthorizationState",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
