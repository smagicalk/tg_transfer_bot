#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the minimum number of Telegram Stars that must be paid by general participant for each sent message to a live story call. Requires groupCall.can_be_managed right
/// # Arguments
/// * `group_call_id` - Group call identifier; must be an identifier of a live story call
/// * `paid_message_star_count` - The new minimum number of Telegram Stars; 0-getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_paid_message_star_count(group_call_id: i32, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallPaidMessageStarCount",
        "group_call_id": group_call_id,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
