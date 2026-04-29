#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes settings for gift receiving for the current user
/// # Arguments
/// * `settings` - The new settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_settings(settings: crate::types::GiftSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGiftSettings",
        "settings": settings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
