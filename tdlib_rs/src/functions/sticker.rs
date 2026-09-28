//!
//! TDLib `sticker` domain functions.
//!
//! Types, enums, and functions for stickers, custom emoji sets, and animated dice.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about an emoji reaction. Returns a 404 error if the reaction is not found
///
/// # Arguments
///
/// * `emoji` - Text representation of the reaction
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiReaction)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_reaction(emoji: String, client_id: i32) -> Result<crate::enums::EmojiReaction, crate::types::Error> {
    let request = json!({
        "@type": "getEmojiReaction",
        "emoji": emoji,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns TGS stickers with generic animations for custom emoji reactions
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_custom_emoji_reaction_animations(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getCustomEmojiReactionAnimations",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an emoji for the flag of the given country. Returns an empty string on failure. Can be called synchronously
///
/// # Arguments
///
/// * `country_code` - A two-letter ISO 3166-1 alpha-2 country code as received from getCountries
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_country_flag_emoji(country_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getCountryFlagEmoji",
        "country_code": country_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that a message with an animated emoji was clicked by the user. Returns a big animated sticker to be played or a 404 error if usual animation needs to be played
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Identifier of the clicked message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Sticker)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn click_animated_emoji_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Sticker, crate::types::Error> {
    let request = json!({
        "@type": "clickAnimatedEmojiMessage",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the emoji status of a chat. Use chatBoostLevelFeatures.can_set_emoji_status to check whether an emoji status can be set. Requires can_change_info administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `emoji_status` - New emoji status; pass null to remove emoji status
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_emoji_status(chat_id: i64, emoji_status: Option<crate::types::EmojiStatus>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatEmojiStatus",
        "chat_id": chat_id,
        "emoji_status": emoji_status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the current state of stake dice
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StakeDiceState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_stake_dice_state(client_id: i32) -> Result<crate::enums::StakeDiceState, crate::types::Error> {
    let request = json!({
        "@type": "getStakeDiceState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns up to 8 emoji statuses, which must be shown right after the default Premium Badge in the emoji status list for self status
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatusCustomEmojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_themed_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getThemedEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns recent emoji statuses for self status
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatuses)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatuses, crate::types::Error> {
    let request = json!({
        "@type": "getRecentEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available upgraded gift emoji statuses for self status
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatuses)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatuses, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default emoji statuses for self status
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatusCustomEmojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Clears the list of recently used emoji statuses for self status
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
pub async fn clear_recent_emoji_statuses(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns up to 8 emoji statuses, which must be shown in the emoji status list for chats
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatusCustomEmojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_themed_chat_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getThemedChatEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default emoji statuses for chats
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatusCustomEmojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_chat_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultChatEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of emoji statuses, which can't be used as chat emoji status, even if they are from a sticker set with is_allowed_as_chat_emoji_status == true
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiStatusCustomEmojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_disallowed_chat_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getDisallowedChatEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Toggles whether the bot can manage emoji status of the current user
///
/// # Arguments
///
/// * `bot_user_id` - User identifier of the bot
/// * `can_manage_emoji_status` - Pass true if the bot is allowed to change emoji status of the user; pass false otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_can_manage_emoji_status(bot_user_id: i64, can_manage_emoji_status: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotCanManageEmojiStatus",
        "bot_user_id": bot_user_id,
        "can_manage_emoji_status": can_manage_emoji_status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the emoji status of a user; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `emoji_status` - New emoji status; pass null to switch to the default badge
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_user_emoji_status(user_id: i64, emoji_status: Option<crate::types::EmojiStatus>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserEmojiStatus",
        "user_id": user_id,
        "emoji_status": emoji_status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns outline of a sticker. This is an offline method. Returns a 404 error if the outline isn't known
///
/// # Arguments
///
/// * `sticker_file_id` - File identifier of the sticker
/// * `for_animated_emoji` - Pass true to get the outline scaled for animated emoji
/// * `for_clicked_animated_emoji_message` - Pass true to get the outline scaled for clicked animated emoji message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Outline)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_outline(sticker_file_id: i32, for_animated_emoji: bool, for_clicked_animated_emoji_message: bool, client_id: i32) -> Result<crate::enums::Outline, crate::types::Error> {
    let request = json!({
        "@type": "getStickerOutline",
        "sticker_file_id": sticker_file_id,
        "for_animated_emoji": for_animated_emoji,
        "for_clicked_animated_emoji_message": for_clicked_animated_emoji_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns outline of a sticker as an SVG path. This is an offline method. Returns an empty string if the outline isn't known
///
/// # Arguments
///
/// * `sticker_file_id` - File identifier of the sticker
/// * `for_animated_emoji` - Pass true to get the outline scaled for animated emoji
/// * `for_clicked_animated_emoji_message` - Pass true to get the outline scaled for clicked animated emoji message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_outline_svg_path(sticker_file_id: i32, for_animated_emoji: bool, for_clicked_animated_emoji_message: bool, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getStickerOutlineSvgPath",
        "sticker_file_id": sticker_file_id,
        "for_animated_emoji": for_animated_emoji,
        "for_clicked_animated_emoji_message": for_clicked_animated_emoji_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns stickers from the installed sticker sets that correspond to any of the given emoji or can be found by sticker-specific keywords. If the query is non-empty, then favorite, recently used or trending stickers may also be returned
///
/// # Arguments
///
/// * `sticker_type` - Type of the stickers to return
/// * `query` - Search query; a space-separated list of emojis or a keyword prefix. If empty, returns all known installed stickers
/// * `limit` - The maximum number of stickers to be returned
/// * `chat_id` - Chat identifier for which to return stickers. Available custom emoji stickers may be different for different chats
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_stickers(sticker_type: crate::enums::StickerType, query: String, limit: i32, chat_id: i64, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getStickers",
        "sticker_type": sticker_type,
        "query": query,
        "limit": limit,
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns unique emoji that correspond to stickers to be found by the getStickers(sticker_type, query, 1000000, chat_id)
///
/// # Arguments
///
/// * `sticker_type` - Type of the stickers to search for
/// * `query` - Search query
/// * `chat_id` - Chat identifier for which to find stickers
/// * `return_only_main_emoji` - Pass true if only main emoji for each found sticker must be included in the result
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Emojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_all_sticker_emojis(sticker_type: crate::enums::StickerType, query: String, chat_id: i64, return_only_main_emoji: bool, client_id: i32) -> Result<crate::enums::Emojis, crate::types::Error> {
    let request = json!({
        "@type": "getAllStickerEmojis",
        "sticker_type": sticker_type,
        "query": query,
        "chat_id": chat_id,
        "return_only_main_emoji": return_only_main_emoji,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for stickers from public sticker sets that correspond to any of the given emoji
///
/// # Arguments
///
/// * `sticker_type` - Type of the stickers to return
/// * `emojis` - Space-separated list of emojis to search for
/// * `query` - Query to search for; may be empty to search for emoji only
/// * `input_language_codes` - List of possible IETF language tags of the user's input language; may be empty if unknown
/// * `offset` - The offset from which to return the stickers; must be non-negative
/// * `limit` - The maximum number of stickers to be returned; 0-100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_stickers(sticker_type: crate::enums::StickerType, emojis: String, query: String, input_language_codes: Vec<String>, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "searchStickers",
        "sticker_type": sticker_type,
        "emojis": emojis,
        "query": query,
        "input_language_codes": input_language_codes,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns greeting stickers from regular sticker sets that can be used for the start page of other users
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_greeting_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getGreetingStickers",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns premium stickers from regular sticker sets
///
/// # Arguments
///
/// * `limit` - The maximum number of stickers to be returned; 0-100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_stickers(limit: i32, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumStickers",
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of installed sticker sets
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_installed_sticker_sets(sticker_type: crate::enums::StickerType, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getInstalledStickerSets",
        "sticker_type": sticker_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of archived sticker sets
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to return
/// * `offset_sticker_set_id` - Identifier of the sticker set from which to return the result; use 0 to get results from the beginning
/// * `limit` - The maximum number of sticker sets to return; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_archived_sticker_sets(sticker_type: crate::enums::StickerType, offset_sticker_set_id: i64, limit: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getArchivedStickerSets",
        "sticker_type": sticker_type,
        "offset_sticker_set_id": offset_sticker_set_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of trending sticker sets. For optimal performance, the number of returned sticker sets is chosen by TDLib
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to return
/// * `offset` - The offset from which to return the sticker sets; must be non-negative
/// * `limit` - The maximum number of sticker sets to be returned; up to 100. For optimal performance, the number of returned sticker sets is chosen by TDLib and can be smaller than the specified limit, even if the end of the list has not been reached
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TrendingStickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_trending_sticker_sets(sticker_type: crate::enums::StickerType, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::TrendingStickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getTrendingStickerSets",
        "sticker_type": sticker_type,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of sticker sets attached to a file, including regular, mask, and emoji sticker sets. Currently, only animations, photos, and videos can have attached sticker sets
///
/// # Arguments
///
/// * `file_id` - File identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_attached_sticker_sets(file_id: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getAttachedStickerSets",
        "file_id": file_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a sticker set by its identifier
///
/// # Arguments
///
/// * `set_id` - Identifier of the sticker set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSet)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_set(set_id: i64, client_id: i32) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
        "@type": "getStickerSet",
        "set_id": set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns name of a sticker set by its identifier
///
/// # Arguments
///
/// * `set_id` - Identifier of the sticker set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_set_name(set_id: i64, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getStickerSetName",
        "set_id": set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for a sticker set by its name
///
/// # Arguments
///
/// * `name` - Name of the sticker set
/// * `ignore_cache` - Pass true to ignore local cache of sticker sets and always send a network request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSet)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_sticker_set(name: String, ignore_cache: bool, client_id: i32) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
        "@type": "searchStickerSet",
        "name": name,
        "ignore_cache": ignore_cache,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for installed sticker sets by looking for specified query in their title and name
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to search for
/// * `query` - Query to search for
/// * `limit` - The maximum number of sticker sets to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_installed_sticker_sets(sticker_type: crate::enums::StickerType, query: String, limit: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "searchInstalledStickerSets",
        "sticker_type": sticker_type,
        "query": query,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to return
/// * `query` - Query to search for
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_sticker_sets(sticker_type: crate::enums::StickerType, query: String, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "searchStickerSets",
        "sticker_type": sticker_type,
        "query": query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Installs/uninstalls or activates/archives a sticker set
///
/// # Arguments
///
/// * `set_id` - Identifier of the sticker set
/// * `is_installed` - The new value of is_installed
/// * `is_archived` - The new value of is_archived. A sticker set can't be installed and archived simultaneously
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn change_sticker_set(set_id: i64, is_installed: bool, is_archived: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "changeStickerSet",
        "set_id": set_id,
        "is_installed": is_installed,
        "is_archived": is_archived,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs the server that some trending sticker sets have been viewed by the user
///
/// # Arguments
///
/// * `sticker_set_ids` - Identifiers of viewed trending sticker sets
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn view_trending_sticker_sets(sticker_set_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewTrendingStickerSets",
        "sticker_set_ids": sticker_set_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the order of installed sticker sets
///
/// # Arguments
///
/// * `sticker_type` - Type of the sticker sets to reorder
/// * `sticker_set_ids` - Identifiers of installed sticker sets in the new correct order
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_installed_sticker_sets(sticker_type: crate::enums::StickerType, sticker_set_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderInstalledStickerSets",
        "sticker_type": sticker_type,
        "sticker_set_ids": sticker_set_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of recently used stickers
///
/// # Arguments
///
/// * `is_attached` - Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_stickers(is_attached: bool, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getRecentStickers",
        "is_attached": is_attached,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first.
/// Only stickers belonging to a sticker set or in WEBP or WEBM format can be added to this list. Emoji stickers can't be added to recent stickers
///
/// # Arguments
///
/// * `is_attached` - Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers
/// * `sticker` - Sticker file to add
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_recent_sticker(is_attached: bool, sticker: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "addRecentSticker",
        "is_attached": is_attached,
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a sticker from the list of recently used stickers
///
/// # Arguments
///
/// * `is_attached` - Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers
/// * `sticker` - Sticker file to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_recent_sticker(is_attached: bool, sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeRecentSticker",
        "is_attached": is_attached,
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Clears the list of recently used stickers
///
/// # Arguments
///
/// * `is_attached` - Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clear_recent_stickers(is_attached: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentStickers",
        "is_attached": is_attached,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns favorite stickers
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_favorite_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getFavoriteStickers",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first.
/// Only stickers belonging to a sticker set or in WEBP or WEBM format can be added to this list. Emoji stickers can't be added to favorite stickers
///
/// # Arguments
///
/// * `sticker` - Sticker file to add
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_favorite_sticker(sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addFavoriteSticker",
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a sticker from the list of favorite stickers
///
/// # Arguments
///
/// * `sticker` - Sticker file to delete from the list
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_favorite_sticker(sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeFavoriteSticker",
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
///
/// # Arguments
///
/// * `sticker` - Sticker file identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Emojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_emojis(sticker: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::Emojis, crate::types::Error> {
    let request = json!({
        "@type": "getStickerEmojis",
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for emojis by keywords. Supported only if the file database is enabled. Order of results is unspecified
///
/// # Arguments
///
/// * `text` - Text to search for
/// * `input_language_codes` - List of possible IETF language tags of the user's input language; may be empty if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiKeywords)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_emojis(text: String, input_language_codes: Vec<String>, client_id: i32) -> Result<crate::enums::EmojiKeywords, crate::types::Error> {
    let request = json!({
        "@type": "searchEmojis",
        "text": text,
        "input_language_codes": input_language_codes,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns emojis matching the keyword. Supported only if the file database is enabled. Order of results is unspecified
///
/// # Arguments
///
/// * `text` - Text to search for
/// * `input_language_codes` - List of possible IETF language tags of the user's input language; may be empty if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Emojis)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_keyword_emojis(text: String, input_language_codes: Vec<String>, client_id: i32) -> Result<crate::enums::Emojis, crate::types::Error> {
    let request = json!({
        "@type": "getKeywordEmojis",
        "text": text,
        "input_language_codes": input_language_codes,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available emoji categories
///
/// # Arguments
///
/// * `r#type` - Type of emoji categories to return; pass null to get default emoji categories
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmojiCategories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_categories(r#type: Option<crate::enums::EmojiCategoryType>, client_id: i32) -> Result<crate::enums::EmojiCategories, crate::types::Error> {
    let request = json!({
        "@type": "getEmojiCategories",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an animated emoji corresponding to a given emoji. Returns a 404 error if the emoji has no animated emoji
///
/// # Arguments
///
/// * `emoji` - The emoji
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AnimatedEmoji)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_animated_emoji(emoji: String, client_id: i32) -> Result<crate::enums::AnimatedEmoji, crate::types::Error> {
    let request = json!({
        "@type": "getAnimatedEmoji",
        "emoji": emoji,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
///
/// # Arguments
///
/// * `language_code` - Language code for which the emoji replacements will be suggested
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_suggestions_url(language_code: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getEmojiSuggestionsUrl",
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of custom emoji stickers by their identifiers. Stickers are returned in arbitrary order. Only found stickers are returned
///
/// # Arguments
///
/// * `custom_emoji_ids` - Identifiers of custom emoji stickers. At most 200 custom emoji stickers can be received simultaneously
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_custom_emoji_stickers(custom_emoji_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getCustomEmojiStickers",
        "custom_emoji_ids": custom_emoji_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default list of custom emoji stickers for placing on a chat photo
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_chat_photo_custom_emoji_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultChatPhotoCustomEmojiStickers",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default list of custom emoji stickers for placing on a profile photo
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_profile_photo_custom_emoji_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultProfilePhotoCustomEmojiStickers",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default list of custom emoji stickers for reply background
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_background_custom_emoji_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultBackgroundCustomEmojiStickers",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the emoji status of the current user; for Telegram Premium users only
///
/// # Arguments
///
/// * `emoji_status` - New emoji status; pass null to switch to the default badge
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_emoji_status(emoji_status: Option<crate::types::EmojiStatus>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setEmojiStatus",
        "emoji_status": emoji_status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the sticker set of a supergroup; requires can_change_info administrator right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `sticker_set_id` - New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_sticker_set(supergroup_id: i64, sticker_set_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupStickerSet",
        "supergroup_id": supergroup_id,
        "sticker_set_id": sticker_set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the custom emoji sticker set of a supergroup; requires can_change_info administrator right. The chat must have at least chatBoostFeatures.min_custom_emoji_sticker_set_boost_level boost level to pass the corresponding color
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `custom_emoji_sticker_set_id` - New value of the custom emoji sticker set identifier for the supergroup. Use 0 to remove the custom emoji sticker set in the supergroup
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_custom_emoji_sticker_set(supergroup_id: i64, custom_emoji_sticker_set_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupCustomEmojiStickerSet",
        "supergroup_id": supergroup_id,
        "custom_emoji_sticker_set_id": custom_emoji_sticker_set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Uploads a file with a sticker; returns the uploaded file
///
/// # Arguments
///
/// * `user_id` - Sticker file owner; ignored for regular users
/// * `sticker_format` - Sticker format
/// * `sticker` - File to upload; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side.
/// See https:core.telegram.org/animated_stickers#technical-requirements for technical requirements
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::File)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn upload_sticker_file(user_id: i64, sticker_format: crate::enums::StickerFormat, sticker: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "uploadStickerFile",
        "user_id": user_id,
        "sticker_format": sticker_format,
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a suggested name for a new sticker set with a given title
///
/// # Arguments
///
/// * `title` - Sticker set title; 1-64 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_suggested_sticker_set_name(title: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getSuggestedStickerSetName",
        "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a name can be used for a new sticker set
///
/// # Arguments
///
/// * `name` - Name to be checked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CheckStickerSetNameResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_sticker_set_name(name: String, client_id: i32) -> Result<crate::enums::CheckStickerSetNameResult, crate::types::Error> {
    let request = json!({
        "@type": "checkStickerSetName",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new sticker set. Returns the newly created sticker set
///
/// # Arguments
///
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `title` - Sticker set title; 1-64 characters
/// * `name` - Sticker set name. Can contain only English letters, digits and underscores. Must end with *"_by_<bot username>"* (*<bot_username>* is case insensitive) for bots; 0-64 characters.
/// If empty, then the name returned by getSuggestedStickerSetName will be used automatically
/// * `sticker_type` - Type of the stickers in the set
/// * `needs_repainting` - Pass true if stickers in the sticker set must be repainted; for custom emoji sticker sets only
/// * `stickers` - List of stickers to be added to the set; 1-200 stickers for custom emoji sticker sets, and 1-120 stickers otherwise. For TGS stickers, uploadStickerFile must be used before the sticker is shown
/// * `source` - Source of the sticker set; may be empty if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSet)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_new_sticker_set(user_id: i64, title: String, name: String, sticker_type: crate::enums::StickerType, needs_repainting: bool, stickers: Vec<crate::types::NewSticker>, source: String, client_id: i32) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
        "@type": "createNewStickerSet",
        "user_id": user_id,
        "title": title,
        "name": name,
        "sticker_type": sticker_type,
        "needs_repainting": needs_repainting,
        "stickers": stickers,
        "source": source,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a new sticker to a set
///
/// # Arguments
///
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user, and contain less than 200 stickers for custom emoji sticker sets and less than 120 otherwise
/// * `sticker` - Sticker to add to the set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_sticker_to_set(user_id: i64, name: String, sticker: crate::types::NewSticker, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addStickerToSet",
        "user_id": user_id,
        "name": name,
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Replaces existing sticker in a set. The function is equivalent to removeStickerFromSet, then addStickerToSet, then setStickerPositionInSet
///
/// # Arguments
///
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `old_sticker` - Sticker to remove from the set
/// * `new_sticker` - Sticker to add to the set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn replace_sticker_in_set(user_id: i64, name: String, old_sticker: crate::enums::InputFile, new_sticker: crate::types::NewSticker, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "replaceStickerInSet",
        "user_id": user_id,
        "name": name,
        "old_sticker": old_sticker,
        "new_sticker": new_sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets a sticker set thumbnail
///
/// # Arguments
///
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `thumbnail` - Thumbnail to set; pass null to remove the sticker set thumbnail
/// * `format` - Format of the thumbnail; pass null if thumbnail is removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_set_thumbnail(user_id: i64, name: String, thumbnail: Option<crate::enums::InputFile>, format: Option<crate::enums::StickerFormat>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerSetThumbnail",
        "user_id": user_id,
        "name": name,
        "thumbnail": thumbnail,
        "format": format,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets a custom emoji sticker set thumbnail
///
/// # Arguments
///
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `custom_emoji_id` - Identifier of the custom emoji from the sticker set, which will be set as sticker set thumbnail; pass 0 to remove the sticker set thumbnail
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_emoji_sticker_set_thumbnail(name: String, custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCustomEmojiStickerSetThumbnail",
        "name": name,
        "custom_emoji_id": custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets a sticker set title
///
/// # Arguments
///
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `title` - New sticker set title
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_set_title(name: String, title: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerSetTitle",
        "name": name,
        "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Completely deletes a sticker set
///
/// # Arguments
///
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_sticker_set(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStickerSet",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the position of a sticker in the set to which it belongs. The sticker set must be owned by the current user
///
/// # Arguments
///
/// * `sticker` - Sticker
/// * `position` - New position of the sticker in the set, 0-based
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_position_in_set(sticker: crate::enums::InputFile, position: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerPositionInSet",
        "sticker": sticker,
        "position": position,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a sticker from the set to which it belongs. The sticker set must be owned by the current user
///
/// # Arguments
///
/// * `sticker` - Sticker to remove from the set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_sticker_from_set(sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeStickerFromSet",
        "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the list of emojis corresponding to a sticker. The sticker must belong to a regular or custom emoji sticker set that is owned by the current user
///
/// # Arguments
///
/// * `sticker` - Sticker
/// * `emojis` - New string with 1-20 emoji corresponding to the sticker
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_emojis(sticker: crate::enums::InputFile, emojis: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerEmojis",
        "sticker": sticker,
        "emojis": emojis,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the list of keywords of a sticker. The sticker must belong to a regular or custom emoji sticker set that is owned by the current user
///
/// # Arguments
///
/// * `sticker` - Sticker
/// * `keywords` - List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_keywords(sticker: crate::enums::InputFile, keywords: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerKeywords",
        "sticker": sticker,
        "keywords": keywords,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the mask position of a mask sticker. The sticker must belong to a mask sticker set that is owned by the current user
///
/// # Arguments
///
/// * `sticker` - Sticker
/// * `mask_position` - Position where the mask is placed; pass null to remove mask position
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_mask_position(sticker: crate::enums::InputFile, mask_position: Option<crate::types::MaskPosition>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerMaskPosition",
        "sticker": sticker,
        "mask_position": mask_position,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns sticker sets owned by the current user
///
/// # Arguments
///
/// * `offset_sticker_set_id` - Identifier of the sticker set from which to return owned sticker sets; use 0 to get results from the beginning
/// * `limit` - The maximum number of sticker sets to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StickerSets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_owned_sticker_sets(offset_sticker_set_id: i64, limit: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getOwnedStickerSets",
        "offset_sticker_set_id": offset_sticker_set_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns examples of premium stickers for demonstration purposes
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_sticker_examples(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumStickerExamples",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the sticker to be used as representation of the Telegram Premium subscription
///
/// # Arguments
///
/// * `month_count` - Number of months the Telegram Premium subscription will be active
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Sticker)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_info_sticker(month_count: i32, client_id: i32) -> Result<crate::enums::Sticker, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumInfoSticker",
        "month_count": month_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

