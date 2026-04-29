#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits a business chat link of the current account. Requires Telegram Business subscription. Returns the edited link
/// # Arguments
/// * `link` - The link to edit
/// * `link_info` - New description of the link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_chat_link(link: String, link_info: crate::types::InputBusinessChatLink, client_id: i32) -> Result<crate::enums::BusinessChatLink, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessChatLink",
        "link": link,
        "link_info": link_info,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
