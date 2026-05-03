use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Imports messages exported from another app
/// # Arguments
/// * `chat_id` - Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info member right
/// * `message_file` - File with messages to import. Only inputFileLocal and inputFileGenerated are supported. The file must not be previously uploaded
/// * `attached_files` - Files used in the imported messages. Only inputFileLocal and inputFileGenerated are supported. The files must not be previously uploaded
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn import_messages(
    chat_id: i64,
    message_file: crate::enums::InputFile,
    attached_files: Vec<crate::enums::InputFile>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "importMessages",
    "chat_id": chat_id,
    "message_file": message_file,
    "attached_files": attached_files,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
