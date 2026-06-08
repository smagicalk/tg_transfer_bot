use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes type of default reaction for the current user
/// # Arguments
/// * `reaction_type` - New type of the default reaction. The paid reaction can't be set as default
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_default_reaction_type(
    reaction_type: crate::enums::ReactionType,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setDefaultReactionType",
    "reaction_type": reaction_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
