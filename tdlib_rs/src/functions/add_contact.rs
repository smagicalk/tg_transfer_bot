#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds a user to the contact list or edits an existing contact by their user identifier
/// # Arguments
/// * `user_id` - Identifier of the user
/// * `contact` - The contact to add or edit; phone number may be empty and needs to be specified only if known
/// * `share_phone_number` - Pass true to share the current user's phone number with the new contact. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed.
    /// Use the field userFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_contact(user_id: i64, contact: crate::types::ImportedContact, share_phone_number: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addContact",
        "user_id": user_id,
        "contact": contact,
        "share_phone_number": share_phone_number,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
