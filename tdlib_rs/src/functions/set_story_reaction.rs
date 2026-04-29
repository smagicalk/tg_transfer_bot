#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes chosen reaction on a story that has already been sent; not supported for live stories
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - The identifier of the story
/// * `reaction_type` - Type of the reaction to set; pass null to remove the reaction. Custom emoji reactions can be used only by Telegram Premium users. Paid reactions can't be set
/// * `update_recent_reactions` - Pass true if the reaction needs to be added to recent reactions
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_story_reaction(story_poster_chat_id: i64, story_id: i32, reaction_type: Option<crate::enums::ReactionType>, update_recent_reactions: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStoryReaction",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "reaction_type": reaction_type,
        "update_recent_reactions": update_recent_reactions,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
