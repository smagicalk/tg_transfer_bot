#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds tasks of a checklist in a message as done or not done
/// # Arguments
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message containing the checklist. Use messageProperties.can_mark_tasks_as_done to check whether the tasks can be marked as done or not done
/// * `marked_as_done_task_ids` - Identifiers of tasks that were marked as done
/// * `marked_as_not_done_task_ids` - Identifiers of tasks that were marked as not done
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn mark_checklist_tasks_as_done(chat_id: i64, message_id: i64, marked_as_done_task_ids: Vec<i32>, marked_as_not_done_task_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "markChecklistTasksAsDone",
        "chat_id": chat_id,
        "message_id": message_id,
        "marked_as_done_task_ids": marked_as_done_task_ids,
        "marked_as_not_done_task_ids": marked_as_not_done_task_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
