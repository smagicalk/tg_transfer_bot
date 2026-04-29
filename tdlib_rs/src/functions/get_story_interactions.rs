#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns interactions with a story. The method can be called only for stories posted on behalf of the current user
/// # Arguments
/// * `story_id` - Story identifier
/// * `query` - Query to search for in names, usernames and titles; may be empty to get all relevant interactions
/// * `only_contacts` - Pass true to get only interactions by contacts; pass false to get all relevant interactions
/// * `prefer_forwards` - Pass true to get forwards and reposts first, then reactions, then other views; pass false to get interactions sorted just by interaction date
/// * `prefer_with_reaction` - Pass true to get interactions with reaction first; pass false to get interactions sorted just by interaction date. Ignored if prefer_forwards == true
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of story interactions to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story_interactions(story_id: i32, query: String, only_contacts: bool, prefer_forwards: bool, prefer_with_reaction: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::StoryInteractions, crate::types::Error> {
    let request = json!({
        "@type": "getStoryInteractions",
        "story_id": story_id,
        "query": query,
        "only_contacts": only_contacts,
        "prefer_forwards": prefer_forwards,
        "prefer_with_reaction": prefer_with_reaction,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
