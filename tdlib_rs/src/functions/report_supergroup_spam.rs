#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports messages in a supergroup as spam; requires administrator rights in the supergroup
/// # Arguments
/// * `supergroup_id` - Supergroup identifier
/// * `message_ids` - Identifiers of messages to report. Use messageProperties.can_report_supergroup_spam to check whether the message can be reported
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_supergroup_spam(supergroup_id: i64, message_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportSupergroupSpam",
        "supergroup_id": supergroup_id,
        "message_ids": message_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
