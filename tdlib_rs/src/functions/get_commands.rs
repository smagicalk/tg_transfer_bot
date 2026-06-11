use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of commands supported by the bot for the given user scope and language; for bots only
/// # Arguments
/// * `scope` - The scope to which the commands are relevant; pass null to get commands in the default bot command scope
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_commands(
    scope: Option<crate::enums::BotCommandScope>,
    language_code: String,
    client_id: i32,
) -> Result<crate::enums::BotCommands, crate::types::Error> {
    let request = json!({
    "@type": "getCommands",
    "scope": scope,
    "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
