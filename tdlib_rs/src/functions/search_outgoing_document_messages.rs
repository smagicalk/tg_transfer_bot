#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for outgoing messages with content of the type messageDocument in all chats except secret chats. Returns the results in reverse chronological order
/// # Arguments
/// * `query` - Query to search for in document file name and message caption
/// * `limit` - The maximum number of messages to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_outgoing_document_messages(query: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchOutgoingDocumentMessages",
        "query": query,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
