#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes imported contacts using the list of contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts.
/// Query result depends on the result of the previous query, so only one query is possible at the same time
/// # Arguments
/// * `contacts` - The new list of contacts to import
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn change_imported_contacts(contacts: Vec<crate::types::ImportedContact>, client_id: i32) -> Result<crate::enums::ImportedContacts, crate::types::Error> {
    let request = json!({
        "@type": "changeImportedContacts",
        "contacts": contacts,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
