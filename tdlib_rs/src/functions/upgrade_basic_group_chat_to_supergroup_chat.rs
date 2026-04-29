#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires owner privileges. Deactivates the original basic group
/// # Arguments
/// * `chat_id` - Identifier of the chat to upgrade
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn upgrade_basic_group_chat_to_supergroup_chat(chat_id: i64, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "upgradeBasicGroupChatToSupergroupChat",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
