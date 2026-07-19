use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the list of commands supported by the bot for the given user scope and language; for bots only
/// # Arguments
/// * `scope` - The scope to which the commands are relevant; pass null to change commands in the default bot command scope
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the commands will be applied to all users from the given scope, for which language there are no dedicated commands
/// * `commands` - List of the bot's commands
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_commands(
    scope: Option<crate::enums::BotCommandScope>,
    language_code: String,
    commands: Vec<crate::types::BotCommand>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setCommands",
    "scope": scope,
    "language_code": language_code,
    "commands": commands,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
