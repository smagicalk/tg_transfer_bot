#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports a sponsored chat to Telegram moderators
/// # Arguments
/// * `sponsored_chat_unique_id` - Unique identifier of the sponsored chat
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_sponsored_chat(sponsored_chat_unique_id: i64, option_id: String, client_id: i32) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
        "@type": "reportSponsoredChat",
        "sponsored_chat_unique_id": sponsored_chat_unique_id,
        "option_id": option_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
