use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes all files from the file download list
/// # Arguments
/// * `only_active` - Pass true to remove only active downloads, including paused
/// * `only_completed` - Pass true to remove only completed downloads
/// * `delete_from_cache` - Pass true to delete the file from the TDLib file cache
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_all_files_from_downloads(
    only_active: bool,
    only_completed: bool,
    delete_from_cache: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeAllFilesFromDownloads",
    "only_active": only_active,
    "only_completed": only_completed,
    "delete_from_cache": delete_from_cache,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
