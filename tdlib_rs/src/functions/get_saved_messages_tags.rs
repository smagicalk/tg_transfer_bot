use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns tags used in Saved Messages or a Saved Messages topic
/// # Arguments
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which tags will be returned; pass 0 to get all Saved Messages tags
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_messages_tags(
    saved_messages_topic_id: i64,
    client_id: i32,
) -> Result<crate::enums::SavedMessagesTags, crate::types::Error> {
    let request = json!({
    "@type": "getSavedMessagesTags",
    "saved_messages_topic_id": saved_messages_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
