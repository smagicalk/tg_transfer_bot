#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the user opened a sponsored chat
/// # Arguments
/// * `sponsored_chat_unique_id` - Unique identifier of the sponsored chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_sponsored_chat(sponsored_chat_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openSponsoredChat",
        "sponsored_chat_unique_id": sponsored_chat_unique_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
