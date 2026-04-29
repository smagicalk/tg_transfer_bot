#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes users from the contact list
/// # Arguments
/// * `user_ids` - Identifiers of users to be deleted
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_contacts(user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeContacts",
        "user_ids": user_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
