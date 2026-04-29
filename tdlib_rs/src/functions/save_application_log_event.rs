#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Saves application log event on the server. Can be called before authorization
/// # Arguments
/// * `r#type` - Event type
/// * `chat_id` - Optional chat identifier, associated with the event
/// * `data` - The log event data
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn save_application_log_event(r#type: String, chat_id: i64, data: crate::enums::JsonValue, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "saveApplicationLogEvent",
        "type": r#type,
        "chat_id": chat_id,
        "data": data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
