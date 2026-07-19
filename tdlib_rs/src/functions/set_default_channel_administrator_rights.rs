use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets default administrator rights for adding the bot to channel chats; for bots only
/// # Arguments
/// * `default_channel_administrator_rights` - Default administrator rights for adding the bot to channels; pass null to remove default rights
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_default_channel_administrator_rights(
    default_channel_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setDefaultChannelAdministratorRights",
    "default_channel_administrator_rights": default_channel_administrator_rights,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
