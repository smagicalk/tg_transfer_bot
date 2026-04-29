#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns TGS stickers with generic animations for custom emoji reactions
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_custom_emoji_reaction_animations(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getCustomEmojiReactionAnimations",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
