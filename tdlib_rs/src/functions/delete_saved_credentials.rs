#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes saved credentials for all payment provider bots
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_credentials(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedCredentials",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
