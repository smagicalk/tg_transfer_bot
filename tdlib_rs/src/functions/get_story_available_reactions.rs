#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns reactions, which can be chosen for a story
/// # Arguments
/// * `row_size` - Number of reaction per row, 5-25
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story_available_reactions(row_size: i32, client_id: i32) -> Result<crate::enums::AvailableReactions, crate::types::Error> {
    let request = json!({
        "@type": "getStoryAvailableReactions",
        "row_size": row_size,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
