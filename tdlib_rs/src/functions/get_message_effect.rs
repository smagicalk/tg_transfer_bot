#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a message effect. Returns a 404 error if the effect is not found
/// # Arguments
/// * `effect_id` - Unique identifier of the effect
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_effect(effect_id: i64, client_id: i32) -> Result<crate::enums::MessageEffect, crate::types::Error> {
    let request = json!({
        "@type": "getMessageEffect",
        "effect_id": effect_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
