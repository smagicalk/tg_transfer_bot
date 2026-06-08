use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Reports a video message advertisement to Telegram moderators
/// # Arguments
/// * `advertisement_unique_id` - Unique identifier of the advertisement
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_video_message_advertisement(
    advertisement_unique_id: i64,
    option_id: String,
    client_id: i32,
) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
    "@type": "reportVideoMessageAdvertisement",
    "advertisement_unique_id": advertisement_unique_id,
    "option_id": option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
