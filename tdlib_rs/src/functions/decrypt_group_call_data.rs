#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Decrypts group call data received by tgcalls
/// # Arguments
/// * `group_call_id` - Group call identifier. The call must not be a video chat
/// * `participant_id` - Identifier of the group call participant, which sent the data
/// * `data_channel` - Data channel for which data was encrypted; pass null if unknown
/// * `data` - Data to decrypt
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn decrypt_group_call_data(group_call_id: i32, participant_id: crate::enums::MessageSender, data_channel: Option<crate::enums::GroupCallDataChannel>, data: String, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "decryptGroupCallData",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "data_channel": data_channel,
        "data": data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
