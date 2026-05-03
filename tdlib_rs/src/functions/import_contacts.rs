use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
/// # Arguments
/// * `contacts` - The list of contacts to import or edit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn import_contacts(
    contacts: Vec<crate::types::ImportedContact>,
    client_id: i32,
) -> Result<crate::enums::ImportedContacts, crate::types::Error> {
    let request = json!({
    "@type": "importContacts",
    "contacts": contacts,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
