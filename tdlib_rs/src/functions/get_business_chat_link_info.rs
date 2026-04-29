#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a business chat link
/// # Arguments
/// * `link_name` - Name of the link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_business_chat_link_info(link_name: String, client_id: i32) -> Result<crate::enums::BusinessChatLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessChatLinkInfo",
        "link_name": link_name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
