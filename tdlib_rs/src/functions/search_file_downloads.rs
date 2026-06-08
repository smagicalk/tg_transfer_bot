use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for files in the file download list or recently downloaded files from the list
/// # Arguments
/// * `query` - Query to search for; may be empty to return all downloaded files
/// * `only_active` - Pass true to search only for active downloads, including paused
/// * `only_completed` - Pass true to search only for completed downloads
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of files to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_file_downloads(
    query: String,
    only_active: bool,
    only_completed: bool,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::FoundFileDownloads, crate::types::Error> {
    let request = json!({
    "@type": "searchFileDownloads",
    "query": query,
    "only_active": only_active,
    "only_completed": only_completed,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
