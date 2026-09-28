//!
//! TDLib `user` domain functions.
//!
//! Types, enums, and functions for user accounts, privacy settings, and contacts.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
///
/// # Arguments
///
/// * `first_name` - The first name of the user; 1-64 characters
/// * `last_name` - The last name of the user; 0-64 characters
/// * `disable_notification` - Pass true to disable notification about the current user joining Telegram for other users that added them to contact list
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn register_user(first_name: String, last_name: String, disable_notification: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "registerUser",
        "first_name": first_name,
        "last_name": last_name,
        "disable_notification": disable_notification,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_me(client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getMe",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a user by their identifier. This is an offline method if the current user is not a bot
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user(user_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getUser",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns full information about a user by their identifier
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UserFullInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_full_info(user_id: i64, client_id: i32) -> Result<crate::enums::UserFullInfo, crate::types::Error> {
    let request = json!({
        "@type": "getUserFullInfo",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Hides the list of contacts that have close birthdays for 24 hours
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn hide_contact_close_birthdays(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "hideContactCloseBirthdays",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a user to the contact list or edits an existing contact by their user identifier
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `contact` - The contact to add or edit; phone number may be empty and needs to be specified only if known
/// * `share_phone_number` - Pass true to share the current user's phone number with the new contact. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed.
/// Use the field userFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
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
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
///
/// # Arguments
///
/// * `contacts` - The list of contacts to import or edit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ImportedContacts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn import_contacts(contacts: Vec<crate::types::ImportedContact>, client_id: i32) -> Result<crate::enums::ImportedContacts, crate::types::Error> {
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

/// Returns all contacts of the user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_contacts(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getContacts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for the specified query in the first names, last names and usernames of the known user contacts
///
/// # Arguments
///
/// * `query` - Query to search for; may be empty to return all contacts
/// * `limit` - The maximum number of users to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_contacts(query: String, limit: i32, client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "searchContacts",
        "query": query,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes users from the contact list
///
/// # Arguments
///
/// * `user_ids` - Identifiers of users to be deleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_contacts(user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeContacts",
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the total number of imported contacts
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_imported_contact_count(client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getImportedContactCount",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes imported contacts using the list of contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts.
/// Query result depends on the result of the previous query, so only one query is possible at the same time
///
/// # Arguments
///
/// * `contacts` - The new list of contacts to import
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ImportedContacts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn change_imported_contacts(contacts: Vec<crate::types::ImportedContact>, client_id: i32) -> Result<crate::enums::ImportedContacts, crate::types::Error> {
    let request = json!({
        "@type": "changeImportedContacts",
        "contacts": contacts,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Clears all imported contacts, contact list remains unchanged
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clear_imported_contacts(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearImportedContacts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes a note of a contact user
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `note` - Note to set for the user; 0-getOption("user_note_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_user_note(user_id: i64, note: crate::types::FormattedText, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserNote",
        "user_id": user_id,
        "note": note,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Suggests a birthdate to another regular user with common messages and allowing non-paid messages
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `birthdate` - Birthdate to suggest
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn suggest_user_birthdate(user_id: i64, birthdate: crate::types::Birthdate, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "suggestUserBirthdate",
        "user_id": user_id,
        "birthdate": birthdate,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches a user by their phone number. Returns a 404 error if the user can't be found
///
/// # Arguments
///
/// * `phone_number` - Phone number to search for
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_user_by_phone_number(phone_number: String, only_local: bool, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "searchUserByPhoneNumber",
        "phone_number": phone_number,
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the editable username of the current user
///
/// # Arguments
///
/// * `username` - The new value of the username. Use an empty string to remove the username. The username can't be completely removed if there is another active or disabled username
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_username(username: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUsername",
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes active state for a username of the current user. The editable username can't be disabled. May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached
///
/// # Arguments
///
/// * `username` - The username to change
/// * `is_active` - Pass true to activate the username; pass false to disable it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_username_is_active(username: String, is_active: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleUsernameIsActive",
        "username": username,
        "is_active": is_active,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes order of active usernames of the current user
///
/// # Arguments
///
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_active_usernames(usernames: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderActiveUsernames",
        "usernames": usernames,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns an HTTPS link, which can be used to get information about the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UserLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_link(client_id: i32) -> Result<crate::enums::UserLink, crate::types::Error> {
    let request = json!({
        "@type": "getUserLink",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches a user by a token from the user's link
///
/// # Arguments
///
/// * `token` - Token to search for
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_user_by_token(token: String, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "searchUserByToken",
        "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a user who can be contacted to get support
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_support_user(client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getSupportUser",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes user privacy settings
///
/// # Arguments
///
/// * `setting` - The privacy setting
/// * `rules` - The new privacy rules
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_user_privacy_setting_rules(setting: crate::enums::UserPrivacySetting, rules: crate::types::UserPrivacySettingRules, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserPrivacySettingRules",
        "setting": setting,
        "rules": rules,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the current privacy settings
///
/// # Arguments
///
/// * `setting` - The privacy setting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UserPrivacySettingRules)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_privacy_setting_rules(setting: crate::enums::UserPrivacySetting, client_id: i32) -> Result<crate::enums::UserPrivacySettingRules, crate::types::Error> {
    let request = json!({
        "@type": "getUserPrivacySettingRules",
        "setting": setting,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns support information for the given user; for Telegram support only
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UserSupportInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_support_info(user_id: i64, client_id: i32) -> Result<crate::enums::UserSupportInfo, crate::types::Error> {
    let request = json!({
        "@type": "getUserSupportInfo",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets support information for the given user; for Telegram support only
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `message` - New information message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UserSupportInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_user_support_info(user_id: i64, message: crate::types::FormattedText, client_id: i32) -> Result<crate::enums::UserSupportInfo, crate::types::Error> {
    let request = json!({
        "@type": "setUserSupportInfo",
        "user_id": user_id,
        "message": message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

