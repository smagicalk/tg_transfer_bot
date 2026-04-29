#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
/// # Arguments
/// * `location` - Location of the map center
/// * `zoom` - Map zoom level; 13-20
/// * `width` - Map width in pixels before applying scale; 16-1024
/// * `height` - Map height in pixels before applying scale; 16-1024
/// * `scale` - Map scale; 1-3
/// * `chat_id` - Identifier of a chat in which the thumbnail will be shown. Use 0 if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_map_thumbnail_file(location: crate::types::Location, zoom: i32, width: i32, height: i32, scale: i32, chat_id: i64, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "getMapThumbnailFile",
        "location": location,
        "zoom": zoom,
        "width": width,
        "height": height,
        "scale": scale,
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
