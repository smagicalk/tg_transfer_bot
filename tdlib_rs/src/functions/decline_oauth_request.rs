use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Declines an OAuth authorization request
/// # Arguments
/// * `url` - URL of the OAuth deep link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn decline_oauth_request(url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "declineOauthRequest",
    "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
