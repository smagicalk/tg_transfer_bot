#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs the server that some trending sticker sets have been viewed by the user
/// # Arguments
/// * `sticker_set_ids` - Identifiers of viewed trending sticker sets
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn view_trending_sticker_sets(sticker_set_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewTrendingStickerSets",
        "sticker_set_ids": sticker_set_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
