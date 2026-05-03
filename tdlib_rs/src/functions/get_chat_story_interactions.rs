use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns interactions with a story posted in a chat. Can be used only if story is posted on behalf of a chat and the user is an administrator in the chat
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - Story identifier
/// * `reaction_type` - Pass the default heart reaction or a suggested reaction type to receive only interactions with the specified reaction type; pass null to receive all interactions; reactionTypePaid isn't supported
/// * `prefer_forwards` - Pass true to get forwards and reposts first, then reactions, then other views; pass false to get interactions sorted just by interaction date
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of story interactions to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_story_interactions(
    story_poster_chat_id: i64,
    story_id: i32,
    reaction_type: Option<crate::enums::ReactionType>,
    prefer_forwards: bool,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::StoryInteractions, crate::types::Error> {
    let request = json!({
    "@type": "getChatStoryInteractions",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "reaction_type": reaction_type,
    "prefer_forwards": prefer_forwards,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
