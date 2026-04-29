#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns t.me URLs recently visited by a newly registered user
/// # Arguments
/// * `referrer` - Google Play referrer to identify the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recently_visited_t_me_urls(referrer: String, client_id: i32) -> Result<crate::enums::TmeUrls, crate::types::Error> {
    let request = json!({
        "@type": "getRecentlyVisitedTMeUrls",
        "referrer": referrer,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
