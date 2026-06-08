use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for the specified query in the first names, last names and usernames of the known user contacts
/// # Arguments
/// * `query` - Query to search for; may be empty to return all contacts
/// * `limit` - The maximum number of users to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_contacts(
    query: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
    "@type": "searchContacts",
    "query": query,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
