#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds tasks to a checklist in a message
/// # Arguments
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message containing the checklist. Use messageProperties.can_add_tasks to check whether the tasks can be added
/// * `tasks` - List of added tasks
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_checklist_tasks(chat_id: i64, message_id: i64, tasks: Vec<crate::types::InputChecklistTask>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChecklistTasks",
        "chat_id": chat_id,
        "message_id": message_id,
        "tasks": tasks,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
