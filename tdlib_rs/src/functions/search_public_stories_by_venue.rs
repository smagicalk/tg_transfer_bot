use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for public stories from the given venue. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// # Arguments
/// * `venue_provider` - Provider of the venue
/// * `venue_id` - Identifier of the venue in the provider database
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of stories to be returned; up to 100. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_public_stories_by_venue(
    venue_provider: String,
    venue_id: String,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::FoundStories, crate::types::Error> {
    let request = json!({
    "@type": "searchPublicStoriesByVenue",
    "venue_provider": venue_provider,
    "venue_id": venue_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
