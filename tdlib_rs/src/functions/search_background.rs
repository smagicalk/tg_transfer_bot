#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for a background by its name
/// # Arguments
/// * `name` - The name of the background
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_background(name: String, client_id: i32) -> Result<crate::enums::Background, crate::types::Error> {
    let request = json!({
        "@type": "searchBackground",
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
