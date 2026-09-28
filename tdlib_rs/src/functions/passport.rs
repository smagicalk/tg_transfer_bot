//!
//! TDLib `passport` domain functions.
//!
//! Types, enums, and functions for Telegram Passport and identity verification documents.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about a bank card
///
/// # Arguments
///
/// * `bank_card_number` - The bank card number
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BankCardInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bank_card_info(bank_card_number: String, client_id: i32) -> Result<crate::enums::BankCardInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBankCardInfo",
        "bank_card_number": bank_card_number,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns one of the available Telegram Passport elements
///
/// # Arguments
///
/// * `r#type` - Telegram Passport element type
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PassportElement)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_element(r#type: crate::enums::PassportElementType, password: String, client_id: i32) -> Result<crate::enums::PassportElement, crate::types::Error> {
    let request = json!({
        "@type": "getPassportElement",
        "type": r#type,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns all available Telegram Passport elements
///
/// # Arguments
///
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PassportElements)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_all_passport_elements(password: String, client_id: i32) -> Result<crate::enums::PassportElements, crate::types::Error> {
    let request = json!({
        "@type": "getAllPassportElements",
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
///
/// # Arguments
///
/// * `element` - Input Telegram Passport element
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PassportElement)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_passport_element(element: crate::enums::InputPassportElement, password: String, client_id: i32) -> Result<crate::enums::PassportElement, crate::types::Error> {
    let request = json!({
        "@type": "setPassportElement",
        "element": element,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a Telegram Passport element
///
/// # Arguments
///
/// * `r#type` - Element type
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_passport_element(r#type: crate::enums::PassportElementType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deletePassportElement",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `errors` - The errors
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_passport_element_errors(user_id: i64, errors: Vec<crate::types::InputPassportElementError>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPassportElementErrors",
        "user_id": user_id,
        "errors": errors,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a Telegram Passport authorization form for sharing data with a service
///
/// # Arguments
///
/// * `bot_user_id` - User identifier of the service's bot
/// * `scope` - Telegram Passport element types requested by the service
/// * `public_key` - Service's public key
/// * `nonce` - Unique request identifier provided by the service
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PassportAuthorizationForm)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_authorization_form(bot_user_id: i64, scope: String, public_key: String, nonce: String, client_id: i32) -> Result<crate::enums::PassportAuthorizationForm, crate::types::Error> {
    let request = json!({
        "@type": "getPassportAuthorizationForm",
        "bot_user_id": bot_user_id,
        "scope": scope,
        "public_key": public_key,
        "nonce": nonce,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
///
/// # Arguments
///
/// * `authorization_form_id` - Authorization form identifier
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PassportElementsWithErrors)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_authorization_form_available_elements(authorization_form_id: i32, password: String, client_id: i32) -> Result<crate::enums::PassportElementsWithErrors, crate::types::Error> {
    let request = json!({
        "@type": "getPassportAuthorizationFormAvailableElements",
        "authorization_form_id": authorization_form_id,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements are going to be reused
///
/// # Arguments
///
/// * `authorization_form_id` - Authorization form identifier
/// * `types` - Types of Telegram Passport elements chosen by user to complete the authorization form
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_passport_authorization_form(authorization_form_id: i32, types: Vec<crate::enums::PassportElementType>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendPassportAuthorizationForm",
        "authorization_form_id": authorization_form_id,
        "types": types,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

