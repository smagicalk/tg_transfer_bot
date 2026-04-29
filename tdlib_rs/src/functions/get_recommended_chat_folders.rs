#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns recommended chat folders for the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recommended_chat_folders(client_id: i32) -> Result<crate::enums::RecommendedChatFolders, crate::types::Error> {
    let request = json!({
        "@type": "getRecommendedChatFolders",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
