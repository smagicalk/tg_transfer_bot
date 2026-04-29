#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns available emoji categories
/// # Arguments
/// * `r#type` - Type of emoji categories to return; pass null to get default emoji categories
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_categories(r#type: Option<crate::enums::EmojiCategoryType>, client_id: i32) -> Result<crate::enums::EmojiCategories, crate::types::Error> {
    let request = json!({
        "@type": "getEmojiCategories",
        "type": r#type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
