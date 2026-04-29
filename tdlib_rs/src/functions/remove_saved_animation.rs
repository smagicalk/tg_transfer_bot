#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes an animation from the list of saved animations
/// # Arguments
/// * `animation` - Animation file to be removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_saved_animation(animation: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeSavedAnimation",
        "animation": animation,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
