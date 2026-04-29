#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a list of public chats of the specified type, owned by the user
/// # Arguments
/// * `r#type` - Type of the public chats to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_created_public_chats(r#type: crate::enums::PublicChatType, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getCreatedPublicChats",
        "type": r#type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
