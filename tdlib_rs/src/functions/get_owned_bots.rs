#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of bots owned by the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_owned_bots(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getOwnedBots",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
