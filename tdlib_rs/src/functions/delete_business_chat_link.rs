#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes a business chat link of the current account
/// # Arguments
/// * `link` - The link to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_chat_link(link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessChatLink",
        "link": link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
