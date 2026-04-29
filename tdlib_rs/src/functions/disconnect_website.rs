#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Disconnects website from the current user's Telegram account
/// # Arguments
/// * `website_id` - Website identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn disconnect_website(website_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disconnectWebsite",
        "website_id": website_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
