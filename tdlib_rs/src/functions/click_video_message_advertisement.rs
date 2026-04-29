#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the user clicked a video message advertisement
/// # Arguments
/// * `advertisement_unique_id` - Unique identifier of the advertisement
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn click_video_message_advertisement(advertisement_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickVideoMessageAdvertisement",
        "advertisement_unique_id": advertisement_unique_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
