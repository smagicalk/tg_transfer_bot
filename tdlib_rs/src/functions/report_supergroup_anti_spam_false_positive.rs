#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports a false deletion of a message by aggressive anti-spam checks; requires administrator rights in the supergroup. Can be called only for messages from chatEventMessageDeleted with can_report_anti_spam_false_positive == true
/// # Arguments
/// * `supergroup_id` - Supergroup identifier
/// * `message_id` - Identifier of the erroneously deleted message from chatEventMessageDeleted
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_supergroup_anti_spam_false_positive(supergroup_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportSupergroupAntiSpamFalsePositive",
        "supergroup_id": supergroup_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
