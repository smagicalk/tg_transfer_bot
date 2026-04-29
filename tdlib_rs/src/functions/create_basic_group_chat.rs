#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an existing chat corresponding to a known basic group
/// # Arguments
/// * `basic_group_id` - Basic group identifier
/// * `force` - Pass true to create the chat without a network request. In this case all information about the chat except its type, title and photo can be incorrect
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_basic_group_chat(basic_group_id: i64, force: bool, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createBasicGroupChat",
        "basic_group_id": basic_group_id,
        "force": force,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
