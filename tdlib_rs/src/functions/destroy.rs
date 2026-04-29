#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed.
/// After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn destroy(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "destroy",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
