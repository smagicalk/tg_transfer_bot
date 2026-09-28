//!
//! TDLib `notification` domain functions.
//!
//! Types, enums, and functions for push notifications and notification settings.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
///
/// # Arguments
///
/// * `notification_group_id` - Identifier of notification group to which the notification belongs
/// * `notification_id` - Identifier of removed notification
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_notification(notification_group_id: i32, notification_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeNotification",
        "notification_group_id": notification_group_id,
        "notification_id": notification_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
///
/// # Arguments
///
/// * `notification_group_id` - Notification group identifier
/// * `max_notification_id` - The maximum identifier of removed notifications
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_notification_group(notification_group_id: i32, max_notification_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeNotificationGroup",
        "notification_group_id": notification_group_id,
        "max_notification_id": max_notification_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `notification_settings` - New notification settings for the chat. If the chat is muted for more than 366 days, it is considered to be muted forever
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_notification_settings(chat_id: i64, notification_settings: crate::types::ChatNotificationSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatNotificationSettings",
        "chat_id": chat_id,
        "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the value of the default disable_notification parameter, used when a message is sent to a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `default_disable_notification` - New value of default_disable_notification
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_default_disable_notification(chat_id: i64, default_disable_notification: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatDefaultDisableNotification",
        "chat_id": chat_id,
        "default_disable_notification": default_disable_notification,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns saved notification sound by its identifier. Returns a 404 error if there is no saved notification sound with the specified identifier
///
/// # Arguments
///
/// * `notification_sound_id` - Identifier of the notification sound
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::NotificationSound)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_notification_sound(notification_sound_id: i64, client_id: i32) -> Result<crate::enums::NotificationSound, crate::types::Error> {
    let request = json!({
        "@type": "getSavedNotificationSound",
        "notification_sound_id": notification_sound_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of saved notification sounds. If a sound isn't in the list, then default sound needs to be used
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::NotificationSounds)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_notification_sounds(client_id: i32) -> Result<crate::enums::NotificationSounds, crate::types::Error> {
    let request = json!({
        "@type": "getSavedNotificationSounds",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a new notification sound to the list of saved notification sounds. The new notification sound is added to the top of the list. If it is already in the list, its position isn't changed
///
/// # Arguments
///
/// * `sound` - Notification sound file to add
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::NotificationSound)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_saved_notification_sound(sound: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::NotificationSound, crate::types::Error> {
    let request = json!({
        "@type": "addSavedNotificationSound",
        "sound": sound,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a notification sound from the list of saved notification sounds
///
/// # Arguments
///
/// * `notification_sound_id` - Identifier of the notification sound
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_saved_notification_sound(notification_sound_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeSavedNotificationSound",
        "notification_sound_id": notification_sound_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of chats with non-default notification settings for new messages
///
/// # Arguments
///
/// * `scope` - If specified, only chats from the scope will be returned; pass null to return chats from all scopes
/// * `compare_sound` - Pass true to include in the response chats with only non-default sound
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_notification_settings_exceptions(scope: Option<crate::enums::NotificationSettingsScope>, compare_sound: bool, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatNotificationSettingsExceptions",
        "scope": scope,
        "compare_sound": compare_sound,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the notification settings for chats of a given type
///
/// # Arguments
///
/// * `scope` - Types of chats for which to return the notification settings information
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ScopeNotificationSettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_scope_notification_settings(scope: crate::enums::NotificationSettingsScope, client_id: i32) -> Result<crate::enums::ScopeNotificationSettings, crate::types::Error> {
    let request = json!({
        "@type": "getScopeNotificationSettings",
        "scope": scope,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes notification settings for chats of a given type
///
/// # Arguments
///
/// * `scope` - Types of chats for which to change the notification settings
/// * `notification_settings` - The new notification settings for the given scope
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_scope_notification_settings(scope: crate::enums::NotificationSettingsScope, notification_settings: crate::types::ScopeNotificationSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setScopeNotificationSettings",
        "scope": scope,
        "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes notification settings for reactions
///
/// # Arguments
///
/// * `notification_settings` - The new notification settings for reactions
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_reaction_notification_settings(notification_settings: crate::types::ReactionNotificationSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setReactionNotificationSettings",
        "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resets all chat and scope notification settings to their default values. By default, all chats are unmuted and message previews are shown
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
pub async fn reset_all_notification_settings(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resetAllNotificationSettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether notifications for new gifts received by a channel chat are sent to the current user; requires can_post_messages administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the channel chat
/// * `are_enabled` - Pass true to enable notifications about new gifts owned by the channel chat; pass false to disable the notifications
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_gift_notifications(chat_id: i64, are_enabled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatGiftNotifications",
        "chat_id": chat_id,
        "are_enabled": are_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
///
/// # Arguments
///
/// * `payload` - JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_push_notification(payload: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processPushNotification",
        "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. Can be called synchronously
///
/// # Arguments
///
/// * `payload` - JSON-encoded push notification payload
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PushReceiverId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_push_receiver_id(payload: String, client_id: i32) -> Result<crate::enums::PushReceiverId, crate::types::Error> {
    let request = json!({
        "@type": "getPushReceiverId",
        "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

