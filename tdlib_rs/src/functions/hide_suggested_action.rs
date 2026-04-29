#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Hides a suggested action
/// # Arguments
/// * `action` - Suggested action to hide
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn hide_suggested_action(action: crate::enums::SuggestedAction, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "hideSuggestedAction",
        "action": action,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
