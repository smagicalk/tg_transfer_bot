use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
/// # Arguments
/// * `size` - Limit on the total size of files after deletion, in bytes. Pass -1 to use the default limit
/// * `ttl` - Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit
/// * `count` - Limit on the total number of files after deletion. Pass -1 to use the default limit
/// * `immunity_delay` - The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value
/// * `file_types` - If non-empty, only files with the given types are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted
/// * `chat_ids` - If non-empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)
/// * `exclude_chat_ids` - If non-empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)
/// * `return_deleted_file_statistics` - Pass true if statistics about the files that were deleted must be returned instead of the whole storage usage statistics. Affects only returned statistics
/// * `chat_limit` - Same as in getStorageStatistics. Affects only returned statistics
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn optimize_storage(
    size: i64,
    ttl: i32,
    count: i32,
    immunity_delay: i32,
    file_types: Vec<crate::enums::FileType>,
    chat_ids: Vec<i64>,
    exclude_chat_ids: Vec<i64>,
    return_deleted_file_statistics: bool,
    chat_limit: i32,
    client_id: i32,
) -> Result<crate::enums::StorageStatistics, crate::types::Error> {
    let request = json!({
    "@type": "optimizeStorage",
    "size": size,
    "ttl": ttl,
    "count": count,
    "immunity_delay": immunity_delay,
    "file_types": file_types,
    "chat_ids": chat_ids,
    "exclude_chat_ids": exclude_chat_ids,
    "return_deleted_file_statistics": return_deleted_file_statistics,
    "chat_limit": chat_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
