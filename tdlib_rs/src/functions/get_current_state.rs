#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns all updates needed to restore current TDLib state, i.e. all actual updateAuthorizationState/updateUser/updateNewChat and others. This is especially useful if TDLib is run in a separate process. Can be called before initialization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_current_state(client_id: i32) -> Result<crate::enums::Updates, crate::types::Error> {
    let request = json!({
        "@type": "getCurrentState",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
