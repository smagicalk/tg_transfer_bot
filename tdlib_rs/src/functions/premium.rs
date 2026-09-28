//!
//! TDLib `premium` domain functions.
//!
//! Types, enums, and functions for Telegram Premium, chat boosts, giveaways, and business features.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Checks whether an in-store purchase of Telegram Premium is possible before authorization. Works only when the current authorization state is authorizationStateWaitPremiumPurchase
///
/// # Arguments
///
/// * `premium_day_count` - The number of days for which the Telegram Premium subscription will be granted
/// * `currency` - ISO 4217 currency code of the payment currency
/// * `amount` - Paid amount, in the smallest units of the currency
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_premium_purchase(premium_day_count: i32, currency: String, amount: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationPremiumPurchase",
        "premium_day_count": premium_day_count,
        "currency": currency,
        "amount": amount,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs server about an in-store purchase of Telegram Premium before authorization. Works only when the current authorization state is authorizationStateWaitPremiumPurchase
///
/// # Arguments
///
/// * `transaction` - Information about the transaction
/// * `is_restore` - Pass true if this is a restore of a Telegram Premium purchase; only for App Store
/// * `premium_day_count` - The number of days for which the Telegram Premium subscription will be granted
/// * `currency` - ISO 4217 currency code of the payment currency
/// * `amount` - Paid amount, in the smallest units of the currency
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_premium_purchase_transaction(transaction: crate::enums::StoreTransaction, is_restore: bool, premium_day_count: i32, currency: String, amount: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAuthenticationPremiumPurchaseTransaction",
        "transaction": transaction,
        "is_restore": is_restore,
        "premium_day_count": premium_day_count,
        "currency": currency,
        "amount": amount,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a message on behalf of a business account; for bots only. Returns the message after it was sent
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `chat_id` - Target chat
/// * `reply_to` - Information about the message to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving
/// * `effect_id` - Identifier of the effect to apply to the message
/// * `reply_markup` - Markup for replying to the message; pass null if none
/// * `input_message_content` - The content of the message to be sent
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_business_message(business_connection_id: String, chat_id: i64, reply_to: Option<crate::enums::InputMessageReplyTo>, disable_notification: bool, protect_content: bool, effect_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "sendBusinessMessage",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "reply_to": reply_to,
        "disable_notification": disable_notification,
        "protect_content": protect_content,
        "effect_id": effect_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends 2-10 messages grouped together into an album on behalf of a business account; for bots only. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `chat_id` - Target chat
/// * `reply_to` - Information about the message to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving
/// * `effect_id` - Identifier of the effect to apply to the message
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_business_message_album(business_connection_id: String, chat_id: i64, reply_to: Option<crate::enums::InputMessageReplyTo>, disable_notification: bool, protect_content: bool, effect_id: i64, input_message_contents: Vec<crate::enums::InputMessageContent>, client_id: i32) -> Result<crate::enums::BusinessMessages, crate::types::Error> {
    let request = json!({
        "@type": "sendBusinessMessageAlbum",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "reply_to": reply_to,
        "disable_notification": disable_notification,
        "protect_content": protect_content,
        "effect_id": effect_id,
        "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the text of a text or game message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `input_message_content` - New text content of the message. Must be of type inputMessageText or inputMessageRichMessage
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_text(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageText",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the content of a live location in a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `location` - New live location of the message; pass null to stop sharing the live location. If the new live_period isn't set to 0x7FFFFFFF,
/// then it must not exceed the current live_period by more than a day, and the live location expiration date must remain in the next 90 days
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_live_location(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, location: Option<crate::types::LiveLocation>, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageLiveLocation",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the content of a checklist in a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `checklist` - The new checklist. If some tasks were completed, this information will be kept
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_checklist(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, checklist: crate::types::InputChecklist, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageChecklist",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "checklist": checklist,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the media content of a message with a text, an animation, an audio, a document, a photo or a video in a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageDocument, inputMessagePhoto or inputMessageVideo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_media(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageMedia",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the caption of a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `caption` - New message content caption; pass null to remove caption; 0-getOption("message_caption_length_max") characters
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_caption(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, caption: Option<crate::types::FormattedText>, show_caption_above_media: bool, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageCaption",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "caption": caption,
        "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the reply markup of a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_message_reply_markup(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessMessageReplyMarkup",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Stops a poll sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message with the poll was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message containing the poll
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn stop_business_poll(business_connection_id: String, chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, client_id: i32) -> Result<crate::enums::BusinessMessage, crate::types::Error> {
    let request = json!({
        "@type": "stopBusinessPoll",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Pins or unpins a message sent on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `is_pinned` - Pass true to pin the message, pass false to unpin it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_message_is_pinned(business_connection_id: String, chat_id: i64, message_id: i64, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessMessageIsPinned",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
        "is_pinned": is_pinned,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reads a message on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection through which the message was received
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_business_message(business_connection_id: String, chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readBusinessMessage",
        "business_connection_id": business_connection_id,
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes messages on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection through which the messages were received
/// * `message_ids` - Identifier of the messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_messages(business_connection_id: String, message_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessMessages",
        "business_connection_id": business_connection_id,
        "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the first and last name of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `first_name` - The new value of the first name for the business account; 1-64 characters
/// * `last_name` - The new value of the optional last name for the business account; 0-64 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_name(business_connection_id: String, first_name: String, last_name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAccountName",
        "business_connection_id": business_connection_id,
        "first_name": first_name,
        "last_name": last_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the bio of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `bio` - The new value of the bio; 0-getOption("bio_length_max") characters without line feeds
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_bio(business_connection_id: String, bio: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAccountBio",
        "business_connection_id": business_connection_id,
        "bio": bio,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes a profile photo of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `photo` - Profile photo to set; pass null to remove the photo
/// * `is_public` - Pass true to set the public photo, which will be visible even if the main photo is hidden by privacy settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_profile_photo(business_connection_id: String, photo: Option<crate::enums::InputChatPhoto>, is_public: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAccountProfilePhoto",
        "business_connection_id": business_connection_id,
        "photo": photo,
        "is_public": is_public,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the editable username of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `username` - The new value of the username
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_username(business_connection_id: String, username: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAccountUsername",
        "business_connection_id": business_connection_id,
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes settings for gift receiving of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `settings` - The new settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_gift_settings(business_connection_id: String, settings: crate::types::GiftSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAccountGiftSettings",
        "business_connection_id": business_connection_id,
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a business connection by its identifier; for bots only
///
/// # Arguments
///
/// * `connection_id` - Identifier of the business connection to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessConnection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_connection(connection_id: String, client_id: i32) -> Result<crate::enums::BusinessConnection, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessConnection",
        "connection_id": connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of features available on the specific chat boost level. This is an offline method
///
/// # Arguments
///
/// * `is_channel` - Pass true to get the list of features for channels; pass false to get the list of features for supergroups
/// * `level` - Chat boost level
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostLevelFeatures)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_level_features(is_channel: bool, level: i32, client_id: i32) -> Result<crate::enums::ChatBoostLevelFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostLevelFeatures",
        "is_channel": is_channel,
        "level": level,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of features available for different chat boost levels. This is an offline method
///
/// # Arguments
///
/// * `is_channel` - Pass true to get the list of features for channels; pass false to get the list of features for supergroups
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostFeatures)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_features(is_channel: bool, client_id: i32) -> Result<crate::enums::ChatBoostFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostFeatures",
        "is_channel": is_channel,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of available chat boost slots for the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostSlots)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_available_chat_boost_slots(client_id: i32) -> Result<crate::enums::ChatBoostSlots, crate::types::Error> {
    let request = json!({
        "@type": "getAvailableChatBoostSlots",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the current boost status for a supergroup or a channel chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostStatus)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_status(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatBoostStatus, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostStatus",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Boosts a chat and returns the list of available chat boost slots for the current user after the boost
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `slot_ids` - Identifiers of boost slots of the current user from which to apply boosts to the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostSlots)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn boost_chat(chat_id: i64, slot_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::ChatBoostSlots, crate::types::Error> {
    let request = json!({
        "@type": "boostChat",
        "chat_id": chat_id,
        "slot_ids": slot_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS link to boost the specified supergroup or channel chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_link(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatBoostLink, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostLink",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a link to boost a chat. Can be called for any internal link of the type internalLinkTypeChatBoost
///
/// # Arguments
///
/// * `url` - The link to boost a chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatBoostLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_link_info(url: String, client_id: i32) -> Result<crate::enums::ChatBoostLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostLinkInfo",
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of boosts applied to a chat; requires administrator rights in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `only_gift_codes` - Pass true to receive only boosts received from gift codes and giveaways created by the chat
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of boosts to be returned; up to 100. For optimal performance, the number of returned boosts can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundChatBoosts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boosts(chat_id: i64, only_gift_codes: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundChatBoosts, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoosts",
        "chat_id": chat_id,
        "only_gift_codes": only_gift_codes,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of boosts applied to a chat by a given user; requires administrator rights in the chat; for bots only
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `user_id` - Identifier of the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundChatBoosts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_user_chat_boosts(chat_id: i64, user_id: i64, client_id: i32) -> Result<crate::enums::FoundChatBoosts, crate::types::Error> {
    let request = json!({
        "@type": "getUserChatBoosts",
        "chat_id": chat_id,
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the business location of the current user. Requires Telegram Business subscription
///
/// # Arguments
///
/// * `location` - The new location of the business; pass null to remove the location
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_location(location: Option<crate::types::BusinessLocation>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessLocation",
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the business opening hours of the current user. Requires Telegram Business subscription
///
/// # Arguments
///
/// * `opening_hours` - The new opening hours of the business; pass null to remove the opening hours; up to 28 time intervals can be specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_opening_hours(opening_hours: Option<crate::types::BusinessOpeningHours>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessOpeningHours",
        "opening_hours": opening_hours,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the business greeting message settings of the current user. Requires Telegram Business subscription
///
/// # Arguments
///
/// * `greeting_message_settings` - The new settings for the greeting message of the business; pass null to disable the greeting message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_greeting_message_settings(greeting_message_settings: Option<crate::types::BusinessGreetingMessageSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessGreetingMessageSettings",
        "greeting_message_settings": greeting_message_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the business away message settings of the current user. Requires Telegram Business subscription
///
/// # Arguments
///
/// * `away_message_settings` - The new settings for the away message of the business; pass null to disable the away message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_away_message_settings(away_message_settings: Option<crate::types::BusinessAwayMessageSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAwayMessageSettings",
        "away_message_settings": away_message_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the business start page of the current user. Requires Telegram Business subscription
///
/// # Arguments
///
/// * `start_page` - The new start page of the business; pass null to remove custom start page
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_start_page(start_page: Option<crate::types::InputBusinessStartPage>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessStartPage",
        "start_page": start_page,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about the business bot that is connected to the current user account. Returns a 404 error if there is no connected bot
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessConnectedBotInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_connected_bot(client_id: i32) -> Result<crate::enums::BusinessConnectedBotInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessConnectedBot",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds or changes business bot that is connected to the current user account
///
/// # Arguments
///
/// * `bot` - Connection settings for the bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_business_connected_bot(bot: crate::types::BusinessConnectedBot, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessConnectedBot",
        "bot": bot,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Confirms an unconfirmed business connection of the current user from another device
///
/// # Arguments
///
/// * `bot_user_id` - User identifier of the bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn confirm_business_connected_bot(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "confirmBusinessConnectedBot",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes the business bot that is connected to the current user account
///
/// # Arguments
///
/// * `bot_user_id` - Unique user identifier for the bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_connected_bot(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessConnectedBot",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Pauses or resumes the connected business bot in a specific chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `is_paused` - Pass true to pause the connected bot in the chat; pass false to resume the bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_business_connected_bot_chat_is_paused(chat_id: i64, is_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBusinessConnectedBotChatIsPaused",
        "chat_id": chat_id,
        "is_paused": is_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes the connected business bot from a specific chat by adding the chat to businessRecipients.excluded_chat_ids
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_business_connected_bot_from_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeBusinessConnectedBotFromChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns business chat links created for the current account
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessChatLinks)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_chat_links(client_id: i32) -> Result<crate::enums::BusinessChatLinks, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessChatLinks",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a business chat link for the current account. Requires Telegram Business subscription. There can be up to getOption("business_chat_link_count_max") links created. Returns the created link
///
/// # Arguments
///
/// * `link_info` - Information about the link to create
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessChatLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_business_chat_link(link_info: crate::types::InputBusinessChatLink, client_id: i32) -> Result<crate::enums::BusinessChatLink, crate::types::Error> {
    let request = json!({
        "@type": "createBusinessChatLink",
        "link_info": link_info,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits a business chat link of the current account. Requires Telegram Business subscription. Returns the edited link
///
/// # Arguments
///
/// * `link` - The link to edit
/// * `link_info` - New description of the link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessChatLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_chat_link(link: String, link_info: crate::types::InputBusinessChatLink, client_id: i32) -> Result<crate::enums::BusinessChatLink, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessChatLink",
        "link": link,
        "link_info": link_info,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a business chat link of the current account
///
/// # Arguments
///
/// * `link` - The link to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_chat_link(link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessChatLink",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a business chat link
///
/// # Arguments
///
/// * `link_name` - Name of the link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessChatLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_chat_link_info(link_name: String, client_id: i32) -> Result<crate::enums::BusinessChatLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessChatLinkInfo",
        "link_name": link_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the number of times the supergroup must be boosted by a user to ignore slow mode and chat permission restrictions; requires can_restrict_members administrator right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `unrestrict_boost_count` - New value of the unrestrict_boost_count supergroup setting; 0-8. Use 0 to remove the setting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_unrestrict_boost_count(supergroup_id: i64, unrestrict_boost_count: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupUnrestrictBoostCount",
        "supergroup_id": supergroup_id,
        "unrestrict_boost_count": unrestrict_boost_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a limit, increased for Premium users. Returns a 404 error if the limit is unknown
///
/// # Arguments
///
/// * `limit_type` - Type of the limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumLimit)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_limit(limit_type: crate::enums::PremiumLimitType, client_id: i32) -> Result<crate::enums::PremiumLimit, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumLimit",
        "limit_type": limit_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about features, available to Premium users
///
/// # Arguments
///
/// * `source` - Source of the request; pass null if the method is called from some non-standard source
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumFeatures)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_features(source: Option<crate::enums::PremiumSource>, client_id: i32) -> Result<crate::enums::PremiumFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumFeatures",
        "source": source,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that the user viewed detailed information about a Premium feature on the Premium features screen
///
/// # Arguments
///
/// * `feature` - The viewed premium feature
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn view_premium_feature(feature: crate::enums::PremiumFeature, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewPremiumFeature",
        "feature": feature,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the user clicked Premium subscription button on the Premium features screen
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
pub async fn click_premium_subscription_button(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickPremiumSubscriptionButton",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns state of Telegram Premium subscription and promotion videos for Premium features
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_state(client_id: i32) -> Result<crate::enums::PremiumState, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a Telegram Premium gift code
///
/// # Arguments
///
/// * `code` - The code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumGiftCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_premium_gift_code(code: String, client_id: i32) -> Result<crate::enums::PremiumGiftCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "checkPremiumGiftCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Applies a Telegram Premium gift code
///
/// # Arguments
///
/// * `code` - The code to apply
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn apply_premium_gift_code(code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "applyPremiumGiftCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Launches a prepaid giveaway
///
/// # Arguments
///
/// * `giveaway_id` - Unique identifier of the prepaid giveaway
/// * `parameters` - Giveaway parameters
/// * `winner_count` - The number of users to receive giveaway prize
/// * `star_count` - The number of Telegram Stars to be distributed through the giveaway; pass 0 for Telegram Premium giveaways
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn launch_prepaid_giveaway(giveaway_id: i64, parameters: crate::types::GiveawayParameters, winner_count: i32, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "launchPrepaidGiveaway",
        "giveaway_id": giveaway_id,
        "parameters": parameters,
        "winner_count": winner_count,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a giveaway
///
/// # Arguments
///
/// * `chat_id` - Identifier of the channel chat which started the giveaway
/// * `message_id` - Identifier of the giveaway or a giveaway winners message in the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiveawayInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_giveaway_info(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::GiveawayInfo, crate::types::Error> {
    let request = json!({
        "@type": "getGiveawayInfo",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes affiliate program for a bot
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with an owned bot for which affiliate program is changed
/// * `parameters` - Parameters of the affiliate program; pass null to close the currently active program. If there is an active program, then commission and program duration can only be increased.
/// If the active program is scheduled to be closed, then it can't be changed anymore
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_affiliate_program(chat_id: i64, parameters: Option<crate::types::AffiliateProgramParameters>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAffiliateProgram",
        "chat_id": chat_id,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches a chat with an affiliate program. Returns the chat if found and the program is active
///
/// # Arguments
///
/// * `username` - Username of the chat
/// * `referrer` - The referrer from an internalLinkTypeChatAffiliateProgram link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_affiliate_program(username: String, referrer: String, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "searchChatAffiliateProgram",
        "username": username,
        "referrer": referrer,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches affiliate programs that can be connected to the given affiliate
///
/// # Arguments
///
/// * `affiliate` - The affiliate for which affiliate programs are searched for
/// * `sort_order` - Sort order for the results
/// * `offset` - Offset of the first affiliate program to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of affiliate programs to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundAffiliatePrograms)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_affiliate_programs(affiliate: crate::enums::AffiliateType, sort_order: crate::enums::AffiliateProgramSortOrder, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundAffiliatePrograms, crate::types::Error> {
    let request = json!({
        "@type": "searchAffiliatePrograms",
        "affiliate": affiliate,
        "sort_order": sort_order,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Connects an affiliate program to the given affiliate. Returns information about the connected affiliate program
///
/// # Arguments
///
/// * `affiliate` - The affiliate to which the affiliate program will be connected
/// * `bot_user_id` - Identifier of the bot, which affiliate program is connected
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ConnectedAffiliateProgram)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn connect_affiliate_program(affiliate: crate::enums::AffiliateType, bot_user_id: i64, client_id: i32) -> Result<crate::enums::ConnectedAffiliateProgram, crate::types::Error> {
    let request = json!({
        "@type": "connectAffiliateProgram",
        "affiliate": affiliate,
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Disconnects an affiliate program from the given affiliate and immediately deactivates its referral link. Returns updated information about the disconnected affiliate program
///
/// # Arguments
///
/// * `affiliate` - The affiliate to which the affiliate program is connected
/// * `url` - The referral link of the affiliate program
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ConnectedAffiliateProgram)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn disconnect_affiliate_program(affiliate: crate::enums::AffiliateType, url: String, client_id: i32) -> Result<crate::enums::ConnectedAffiliateProgram, crate::types::Error> {
    let request = json!({
        "@type": "disconnectAffiliateProgram",
        "affiliate": affiliate,
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an affiliate program that was connected to the given affiliate by identifier of the bot that created the program
///
/// # Arguments
///
/// * `affiliate` - The affiliate to which the affiliate program will be connected
/// * `bot_user_id` - Identifier of the bot that created the program
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ConnectedAffiliateProgram)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_connected_affiliate_program(affiliate: crate::enums::AffiliateType, bot_user_id: i64, client_id: i32) -> Result<crate::enums::ConnectedAffiliateProgram, crate::types::Error> {
    let request = json!({
        "@type": "getConnectedAffiliateProgram",
        "affiliate": affiliate,
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns affiliate programs that were connected to the given affiliate
///
/// # Arguments
///
/// * `affiliate` - The affiliate to which the affiliate programs were connected
/// * `offset` - Offset of the first affiliate program to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of affiliate programs to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ConnectedAffiliatePrograms)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_connected_affiliate_programs(affiliate: crate::enums::AffiliateType, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::ConnectedAffiliatePrograms, crate::types::Error> {
    let request = json!({
        "@type": "getConnectedAffiliatePrograms",
        "affiliate": affiliate,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about features, available to Business users
///
/// # Arguments
///
/// * `source` - Source of the request; pass null if the method is called from settings or some non-standard source
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BusinessFeatures)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_features(source: Option<crate::enums::BusinessFeature>, client_id: i32) -> Result<crate::enums::BusinessFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessFeatures",
        "source": source,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

