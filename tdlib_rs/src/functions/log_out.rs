#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn log_out(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "logOut",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
