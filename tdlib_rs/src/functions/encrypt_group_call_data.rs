#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Encrypts group call data before sending them over network using tgcalls
/// # Arguments
/// * `group_call_id` - Group call identifier. The call must not be a video chat
/// * `data_channel` - Data channel for which data is encrypted
/// * `data` - Data to encrypt
/// * `unencrypted_prefix_size` - Size of data prefix that must be kept unencrypted
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn encrypt_group_call_data(group_call_id: i32, data_channel: crate::enums::GroupCallDataChannel, data: String, unencrypted_prefix_size: i32, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "encryptGroupCallData",
        "group_call_id": group_call_id,
        "data_channel": data_channel,
        "data": data,
        "unencrypted_prefix_size": unencrypted_prefix_size,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
