//!
//! TDLib `chat` domain functions.
//!
//! Types, enums, and functions for managing private chats, basic groups, supergroups, and channels.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about a basic group by its identifier. This is an offline method if the current user is not a bot
///
/// # Arguments
///
/// * `basic_group_id` - Basic group identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BasicGroup)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_basic_group(basic_group_id: i64, client_id: i32) -> Result<crate::enums::BasicGroup, crate::types::Error> {
    let request = json!({
        "@type": "getBasicGroup",
        "basic_group_id": basic_group_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns full information about a basic group by its identifier
///
/// # Arguments
///
/// * `basic_group_id` - Basic group identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BasicGroupFullInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_basic_group_full_info(basic_group_id: i64, client_id: i32) -> Result<crate::enums::BasicGroupFullInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBasicGroupFullInfo",
        "basic_group_id": basic_group_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a supergroup or a channel by its identifier. This is an offline method if the current user is not a bot
///
/// # Arguments
///
/// * `supergroup_id` - Supergroup or channel identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Supergroup)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_supergroup(supergroup_id: i64, client_id: i32) -> Result<crate::enums::Supergroup, crate::types::Error> {
    let request = json!({
        "@type": "getSupergroup",
        "supergroup_id": supergroup_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
///
/// # Arguments
///
/// * `supergroup_id` - Supergroup or channel identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::SupergroupFullInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_supergroup_full_info(supergroup_id: i64, client_id: i32) -> Result<crate::enums::SupergroupFullInfo, crate::types::Error> {
    let request = json!({
        "@type": "getSupergroupFullInfo",
        "supergroup_id": supergroup_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a secret chat by its identifier. This is an offline method
///
/// # Arguments
///
/// * `secret_chat_id` - Secret chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::SecretChat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_secret_chat(secret_chat_id: i32, client_id: i32) -> Result<crate::enums::SecretChat, crate::types::Error> {
    let request = json!({
        "@type": "getSecretChat",
        "secret_chat_id": secret_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a chat by its identifier. This is an offline method if the current user is not a bot
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat(chat_id: i64, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "getChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Loads more chats from a chat list. The loaded chats and their positions in the chat list will be sent through updates. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. Returns a 404 error if all chats have been loaded
///
/// # Arguments
///
/// * `chat_list` - The chat list in which to load chats; pass null to load chats from the main chat list
/// * `limit` - The maximum number of chats to be loaded. For optimal performance, the number of loaded chats is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_chats(chat_list: Option<crate::enums::ChatList>, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadChats",
        "chat_list": chat_list,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns an ordered list of chats from the beginning of a chat list. For informational purposes only. Use loadChats and updates processing instead to maintain chat lists in a consistent state
///
/// # Arguments
///
/// * `chat_list` - The chat list in which to return chats; pass null to get chats from the main chat list
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chats(chat_list: Option<crate::enums::ChatList>, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChats",
        "chat_list": chat_list,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches a public chat by its username. Currently, only private chats, supergroups and channels can be public. Returns the chat if found; otherwise, an error is returned
///
/// # Arguments
///
/// * `username` - Username to be resolved
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_chat(username: String, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicChat",
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches public chats by looking for specified query in their username and title. Currently, only private chats, supergroups and channels can be public. Returns a meaningful number of results.
/// Excludes private chats with contacts and chats from the chat list from the results
///
/// # Arguments
///
/// * `query` - Query to search for
/// * `type_filter` - Additional filter for type of the chats to be returned; pass null to search for chats of all types
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_chats(query: String, type_filter: Option<crate::enums::SearchChatTypeFilter>, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicChats",
        "query": query,
        "type_filter": type_filter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for the specified query in the title and username of already known chats. This is an offline method. Returns chats in the order seen in the main chat list
///
/// # Arguments
///
/// * `query` - Query to search for. If the query is empty, returns up to 50 recently found chats
/// * `type_filter` - Additional filter for type of the chats to be returned; pass null to search for chats of all types
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chats(query: String, type_filter: Option<crate::enums::SearchChatTypeFilter>, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchChats",
        "query": query,
        "type_filter": type_filter,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the main chat list
///
/// # Arguments
///
/// * `query` - Query to search for
/// * `type_filter` - Additional filter for type of the chats to be returned; pass null to search for chats of all types
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chats_on_server(query: String, type_filter: Option<crate::enums::SearchChatTypeFilter>, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchChatsOnServer",
        "query": query,
        "type_filter": type_filter,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of channel chats recommended to the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recommended_chats(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getRecommendedChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of chats similar to the given chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the target chat; must be an identifier of a channel chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_similar_chats(chat_id: i64, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatSimilarChats",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns approximate number of chats similar to the given chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the target chat; must be an identifier of a channel chat
/// * `return_local` - Pass true to get the number of chats without sending network requests, or -1 if the number of chats is unknown locally
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_similar_chat_count(chat_id: i64, return_local: bool, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatSimilarChatCount",
        "chat_id": chat_id,
        "return_local": return_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that a chat was opened from the list of similar chats. The method is independent of openChat and closeChat methods
///
/// # Arguments
///
/// * `chat_id` - Identifier of the original chat, which similar chats were requested
/// * `opened_chat_id` - Identifier of the opened chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_chat_similar_chat(chat_id: i64, opened_chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openChatSimilarChat",
        "chat_id": chat_id,
        "opened_chat_id": opened_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of frequently used chats
///
/// # Arguments
///
/// * `category` - Category of chats to be returned
/// * `limit` - The maximum number of chats to be returned; up to 30
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_top_chats(category: crate::enums::TopChatCategory, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getTopChats",
        "category": category,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
///
/// # Arguments
///
/// * `category` - Category of frequently used chats
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_top_chat(category: crate::enums::TopChatCategory, chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeTopChat",
        "category": category,
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches for the specified query in the title and username of up to 50 recently found chats. This is an offline method
///
/// # Arguments
///
/// * `query` - Query to search for
/// * `type_filter` - Additional filter for type of the chats to be returned; pass null to search for chats of all types
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_recently_found_chats(query: String, type_filter: Option<crate::enums::SearchChatTypeFilter>, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchRecentlyFoundChats",
        "query": query,
        "type_filter": type_filter,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to add
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_recently_found_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addRecentlyFoundChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a chat from the list of recently found chats
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to be removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_recently_found_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeRecentlyFoundChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Clears the list of recently found chats
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
pub async fn clear_recently_found_chats(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentlyFoundChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns recently opened chats. This is an offline method. Returns chats in the order of last opening
///
/// # Arguments
///
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recently_opened_chats(limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getRecentlyOpenedChats",
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a username can be set for a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier; must be identifier of a supergroup chat, or a channel chat, or a private chat with self, or 0 if the chat is being created
/// * `username` - Username to be checked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CheckChatUsernameResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_chat_username(chat_id: i64, username: String, client_id: i32) -> Result<crate::enums::CheckChatUsernameResult, crate::types::Error> {
    let request = json!({
        "@type": "checkChatUsername",
        "chat_id": chat_id,
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of public chats of the specified type, owned by the user
///
/// # Arguments
///
/// * `r#type` - Type of the public chats to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_created_public_chats(r#type: crate::enums::PublicChatType, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getCreatedPublicChats",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached. The limit can be increased with Telegram Premium
///
/// # Arguments
///
/// * `r#type` - Type of the public chats, for which to check the limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_created_public_chats_limit(r#type: crate::enums::PublicChatType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkCreatedPublicChatsLimit",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Returned basic group chats must be first upgraded to supergroups before they can be set as a discussion group.
/// To set a returned supergroup as a discussion group, access to its old messages must be enabled using toggleSupergroupIsAllHistoryAvailable first
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_suitable_discussion_chats(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getSuitableDiscussionChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives the error "CHANNELS_TOO_MUCH". Also, the limit can be increased with Telegram Premium
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_inactive_supergroup_chats(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getInactiveSupergroupChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of channel chats, which can be used as a personal chat
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_suitable_personal_chats(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getSuitablePersonalChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a chat along with all messages in the corresponding chat for all chat members. For group chats this will release the usernames and remove all members.
/// Use the field chat.can_be_deleted_for_all_users to find whether the method can be applied to the chat
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
pub async fn delete_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns sponsored chats to be shown in the search results
///
/// # Arguments
///
/// * `query` - Query the user searches for
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::SponsoredChats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_search_sponsored_chats(query: String, client_id: i32) -> Result<crate::enums::SponsoredChats, crate::types::Error> {
    let request = json!({
        "@type": "getSearchSponsoredChats",
        "query": query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that the user fully viewed a sponsored chat
///
/// # Arguments
///
/// * `sponsored_chat_unique_id` - Unique identifier of the sponsored chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn view_sponsored_chat(sponsored_chat_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewSponsoredChat",
        "sponsored_chat_unique_id": sponsored_chat_unique_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the user opened a sponsored chat
///
/// # Arguments
///
/// * `sponsored_chat_unique_id` - Unique identifier of the sponsored chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_sponsored_chat(sponsored_chat_unique_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openSponsoredChat",
        "sponsored_chat_unique_id": sponsored_chat_unique_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports a sponsored chat to Telegram moderators
///
/// # Arguments
///
/// * `sponsored_chat_unique_id` - Unique identifier of the sponsored chat
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReportSponsoredResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_sponsored_chat(sponsored_chat_unique_id: i64, option_id: String, client_id: i32) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
        "@type": "reportSponsoredChat",
        "sponsored_chat_unique_id": sponsored_chat_unique_id,
        "option_id": option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the result of a chat join query; for bots only
///
/// # Arguments
///
/// * `query_id` - Identifier of the query
/// * `result` - The result
/// * `url` - URL of the Web App to open
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_chat_join_request_query(query_id: i64, result: crate::enums::ChatJoinRequestResult, url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerChatJoinRequestQuery",
        "query_id": query_id,
        "result": result,
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a notification about user activity in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `topic_id` - Identifier of the topic in which the action is performed; pass null if none
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `action` - The action description; pass null to cancel the currently active action
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_chat_action(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, business_connection_id: String, action: Option<crate::enums::ChatAction>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendChatAction",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "business_connection_id": business_connection_id,
        "action": action,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
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
pub async fn open_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
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
pub async fn close_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all mentions in a chat as read
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
pub async fn read_all_chat_mentions(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllChatMentions",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns an existing chat corresponding to a given user
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `force` - Pass true to create the chat without a network request. In this case all information about the chat except its type, title and photo can be incorrect
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_private_chat(user_id: i64, force: bool, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createPrivateChat",
        "user_id": user_id,
        "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an existing chat corresponding to a known basic group
///
/// # Arguments
///
/// * `basic_group_id` - Basic group identifier
/// * `force` - Pass true to create the chat without a network request. In this case all information about the chat except its type, title and photo can be incorrect
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_basic_group_chat(basic_group_id: i64, force: bool, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createBasicGroupChat",
        "basic_group_id": basic_group_id,
        "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an existing chat corresponding to a known supergroup or channel
///
/// # Arguments
///
/// * `supergroup_id` - Supergroup or channel identifier
/// * `force` - Pass true to create the chat without a network request. In this case all information about the chat except its type, title and photo can be incorrect
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_supergroup_chat(supergroup_id: i64, force: bool, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createSupergroupChat",
        "supergroup_id": supergroup_id,
        "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an existing chat corresponding to a known secret chat
///
/// # Arguments
///
/// * `secret_chat_id` - Secret chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_secret_chat(secret_chat_id: i32, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createSecretChat",
        "secret_chat_id": secret_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns information about the newly created chat
///
/// # Arguments
///
/// * `user_ids` - Identifiers of users to be added to the basic group; may be empty to create a basic group without other members
/// * `title` - Title of the new basic group; 1-128 characters
/// * `message_auto_delete_time` - Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CreatedBasicGroupChat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_new_basic_group_chat(user_ids: Vec<i64>, title: String, message_auto_delete_time: i32, client_id: i32) -> Result<crate::enums::CreatedBasicGroupChat, crate::types::Error> {
    let request = json!({
        "@type": "createNewBasicGroupChat",
        "user_ids": user_ids,
        "title": title,
        "message_auto_delete_time": message_auto_delete_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
///
/// # Arguments
///
/// * `title` - Title of the new chat; 1-128 characters
/// * `is_forum` - Pass true to create a forum supergroup chat
/// * `is_channel` - Pass true to create a channel chat; ignored if a forum is created
/// * `description` - Chat description; 0-255 characters
/// * `location` - Chat location if a location-based supergroup is being created; pass null to create an ordinary supergroup chat
/// * `message_auto_delete_time` - Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `for_import` - Pass true to create a supergroup for importing messages using importMessages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_new_supergroup_chat(title: String, is_forum: bool, is_channel: bool, description: String, location: Option<crate::types::ChatLocation>, message_auto_delete_time: i32, for_import: bool, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createNewSupergroupChat",
        "title": title,
        "is_forum": is_forum,
        "is_channel": is_channel,
        "description": description,
        "location": location,
        "message_auto_delete_time": message_auto_delete_time,
        "for_import": for_import,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new secret chat. Returns the newly created chat
///
/// # Arguments
///
/// * `user_id` - Identifier of the target user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_new_secret_chat(user_id: i64, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "createNewSecretChat",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires owner privileges. Deactivates the original basic group
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to upgrade
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chat)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn upgrade_basic_group_chat_to_supergroup_chat(chat_id: i64, client_id: i32) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
        "@type": "upgradeBasicGroupChatToSupergroupChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns chat lists to which the chat can be added. This is an offline method
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatLists)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_lists_to_add_chat(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatLists, crate::types::Error> {
    let request = json!({
        "@type": "getChatListsToAddChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a chat to a chat list. A chat can't be simultaneously in Main and Archive chat lists, so it is automatically removed from another one if needed
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `chat_list` - The chat list. Use getChatListsToAddChat to get suitable chat lists
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_to_list(chat_id: i64, chat_list: crate::enums::ChatList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChatToList",
        "chat_id": chat_id,
        "chat_list": chat_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a chat folder by its identifier
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolder)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::ChatFolder, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolder",
        "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates new chat folder. Returns information about the created chat folder. There can be up to getOption("chat_folder_count_max") chat folders, but the limit can be increased with Telegram Premium
///
/// # Arguments
///
/// * `folder` - The new chat folder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_folder(folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::ChatFolderInfo, crate::types::Error> {
    let request = json!({
        "@type": "createChatFolder",
        "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits existing chat folder. Returns information about the edited chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `folder` - The edited chat folder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_folder(chat_folder_id: i32, folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::ChatFolderInfo, crate::types::Error> {
    let request = json!({
        "@type": "editChatFolder",
        "chat_folder_id": chat_folder_id,
        "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes existing chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `leave_chat_ids` - Identifiers of the chats to leave. The chats must be pinned or always included in the folder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_folder(chat_folder_id: i32, leave_chat_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatFolder",
        "chat_folder_id": chat_folder_id,
        "leave_chat_ids": leave_chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns identifiers of pinned or always included chats from a chat folder, which are suggested to be left when the chat folder is deleted
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_chats_to_leave(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderChatsToLeave",
        "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns approximate number of chats in a being created chat folder. Main and archive chat lists must be fully preloaded for this function to work correctly
///
/// # Arguments
///
/// * `folder` - The new chat folder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_chat_count(folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderChatCount",
        "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the order of chat folders
///
/// # Arguments
///
/// * `chat_folder_ids` - Identifiers of chat folders in the new correct order
/// * `main_chat_list_position` - Position of the main chat list among chat folders, 0-based. Can be non-zero only for Premium users
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_chat_folders(chat_folder_ids: Vec<i32>, main_chat_list_position: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderChatFolders",
        "chat_folder_ids": chat_folder_ids,
        "main_chat_list_position": main_chat_list_position,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether chat folder tags are enabled
///
/// # Arguments
///
/// * `are_tags_enabled` - Pass true to enable folder tags; pass false to disable them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_folder_tags(are_tags_enabled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatFolderTags",
        "are_tags_enabled": are_tags_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns recommended chat folders for the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RecommendedChatFolders)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recommended_chat_folders(client_id: i32) -> Result<crate::enums::RecommendedChatFolders, crate::types::Error> {
    let request = json!({
        "@type": "getRecommendedChatFolders",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns default icon name for a folder. Can be called synchronously
///
/// # Arguments
///
/// * `folder` - Chat folder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderIcon)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_default_icon_name(folder: crate::types::ChatFolder, client_id: i32) -> Result<crate::enums::ChatFolderIcon, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderDefaultIconName",
        "folder": folder,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns identifiers of chats from a chat folder, suitable for adding to a chat folder invite link
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chats_for_chat_folder_invite_link(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatsForChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new invite link for a chat folder. A link can be created for a chat folder if it has only pinned and included chats
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `name` - Name of the link; 0-32 characters
/// * `chat_ids` - Identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link creation
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_folder_invite_link(chat_folder_id: i32, name: String, chat_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::ChatFolderInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
        "name": name,
        "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns invite links created by the current user for a shareable chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInviteLinks)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_invite_links(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::ChatFolderInviteLinks, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderInviteLinks",
        "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits an invite link for a chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `invite_link` - Invite link to be edited
/// * `name` - New name of the link; 0-32 characters
/// * `chat_ids` - New identifiers of chats to be accessible by the invite link. Use getChatsForChatFolderInviteLink to get suitable chats. Basic groups will be automatically converted to supergroups before link editing
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_folder_invite_link(chat_folder_id: i32, invite_link: String, name: String, chat_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::ChatFolderInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "editChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
        "invite_link": invite_link,
        "name": name,
        "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes an invite link for a chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `invite_link` - Invite link to be deleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_folder_invite_link(chat_folder_id: i32, invite_link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatFolderInviteLink",
        "chat_folder_id": chat_folder_id,
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks the validity of an invite link for a chat folder and returns information about the corresponding chat folder
///
/// # Arguments
///
/// * `invite_link` - Invite link to be checked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatFolderInviteLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_chat_folder_invite_link(invite_link: String, client_id: i32) -> Result<crate::enums::ChatFolderInviteLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "checkChatFolderInviteLink",
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a chat folder by an invite link
///
/// # Arguments
///
/// * `invite_link` - Invite link for the chat folder
/// * `chat_ids` - Identifiers of the chats added to the chat folder. The chats are automatically joined if they aren't joined yet
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_folder_by_invite_link(invite_link: String, chat_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChatFolderByInviteLink",
        "invite_link": invite_link,
        "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns new chats added to a shareable chat folder by its owner. The method must be called at most once in getOption("chat_folder_new_chats_update_period") for the given chat folder
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_folder_new_chats(chat_folder_id: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatFolderNewChats",
        "chat_folder_id": chat_folder_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Process new chats added to a shareable chat folder by its owner
///
/// # Arguments
///
/// * `chat_folder_id` - Chat folder identifier
/// * `added_chat_ids` - Identifiers of the new chats, which are added to the chat folder. The chats are automatically joined if they aren't joined yet
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_folder_new_chats(chat_folder_id: i32, added_chat_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processChatFolderNewChats",
        "chat_folder_id": chat_folder_id,
        "added_chat_ids": added_chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns settings for automatic moving of chats to and from the Archive chat lists
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ArchiveChatListSettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_archive_chat_list_settings(client_id: i32) -> Result<crate::enums::ArchiveChatListSettings, crate::types::Error> {
    let request = json!({
        "@type": "getArchiveChatListSettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes settings for automatic moving of chats to and from the Archive chat lists
///
/// # Arguments
///
/// * `settings` - New settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_archive_chat_list_settings(settings: crate::types::ArchiveChatListSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setArchiveChatListSettings",
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info member right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `title` - New title of the chat; 1-128 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_title(chat_id: i64, title: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatTitle",
        "chat_id": chat_id,
        "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes accent color and background custom emoji of a channel chat. Requires can_change_info administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `accent_color_id` - Identifier of the accent color to use. The chat must have at least accentColor.min_channel_chat_boost_level boost level to pass the corresponding color
/// * `background_custom_emoji_id` - Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none. Use chatBoostLevelFeatures.can_set_background_custom_emoji to check whether a custom emoji can be set
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_accent_color(chat_id: i64, accent_color_id: i32, background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAccentColor",
        "chat_id": chat_id,
        "accent_color_id": accent_color_id,
        "background_custom_emoji_id": background_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `permissions` - New non-administrator members permissions in the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_permissions(chat_id: i64, permissions: crate::types::ChatPermissions, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPermissions",
        "chat_id": chat_id,
        "permissions": permissions,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the background in a specific chat. Supported only in private and secret chats with non-deleted users, and in chats with sufficient boost level and can_change_info administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `background` - The input background to use; pass null to create a new filled or chat theme background
/// * `r#type` - Background type; pass null to use default background type for the chosen background; backgroundTypeChatTheme isn't supported for private and secret chats.
/// Use chatBoostLevelFeatures.chat_theme_background_count and chatBoostLevelFeatures.can_set_custom_background to check whether the background type can be set in the boosted chat
/// * `dark_theme_dimming` - Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
/// * `only_for_self` - Pass true to set background only for self; pass false to set background for all chat users. Always false for backgrounds set in boosted chats. Background can be set for both users only by Telegram Premium users and if set background isn't of the type inputBackgroundPrevious
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_background(chat_id: i64, background: Option<crate::enums::InputBackground>, r#type: Option<crate::enums::BackgroundType>, dark_theme_dimming: i32, only_for_self: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatBackground",
        "chat_id": chat_id,
        "background": background,
        "type": r#type,
        "dark_theme_dimming": dark_theme_dimming,
        "only_for_self": only_for_self,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes background in a specific chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `restore_previous` - Pass true to restore previously set background. Can be used only in private and secret chats with non-deleted users if userFullInfo.set_chat_background == true.
/// Supposed to be used from messageChatSetBackground messages with the currently set background that was set for both sides by the other user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_background(chat_id: i64, restore_previous: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatBackground",
        "chat_id": chat_id,
        "restore_previous": restore_previous,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns available to the current user gift chat themes
///
/// # Arguments
///
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of chat themes to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftChatThemes)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_chat_themes(offset: String, limit: i32, client_id: i32) -> Result<crate::enums::GiftChatThemes, crate::types::Error> {
    let request = json!({
        "@type": "getGiftChatThemes",
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the chat theme. Supported only in private and secret chats
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `theme` - New chat theme; pass null to return the default theme
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_theme(chat_id: i64, theme: Option<crate::enums::InputChatTheme>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatTheme",
        "chat_id": chat_id,
        "theme": theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the ability of users to save, forward, or copy chat content. Requires owner privileges in basic groups, supergroups and channels.
/// Requires Telegram Premium to enable protected content in private chats. Not available in Saved Messages and private chats with bots or support accounts
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `has_protected_content` - New value of has_protected_content
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_has_protected_content(chat_id: i64, has_protected_content: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatHasProtectedContent",
        "chat_id": chat_id,
        "has_protected_content": has_protected_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Processes request to disable has_protected_content in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `request_message_id` - Identifier of the message with the request. The message must be incoming and has content of the type messageChatHasProtectedContentDisableRequested
/// * `approve` - Pass true to approve the request; pass false to reject the request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_has_protected_content_disable_request(chat_id: i64, request_message_id: i64, approve: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processChatHasProtectedContentDisableRequest",
        "chat_id": chat_id,
        "request_message_id": request_message_id,
        "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the translatable state of a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `is_translatable` - New value of is_translatable
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_translatable(chat_id: i64, is_translatable: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatIsTranslatable",
        "chat_id": chat_id,
        "is_translatable": is_translatable,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the marked as unread state of a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `is_marked_as_unread` - New value of is_marked_as_unread
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_marked_as_unread(chat_id: i64, is_marked_as_unread: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatIsMarkedAsUnread",
        "chat_id": chat_id,
        "is_marked_as_unread": is_marked_as_unread,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes application-specific data associated with a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_data` - New value of client_data
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_client_data(chat_id: i64, client_data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatClientData",
        "chat_id": chat_id,
        "client_data": client_data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info member right
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `description` - New chat description; 0-255 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_description(chat_id: i64, description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDescription",
        "chat_id": chat_id,
        "description": description,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the discussion group of a channel chat; requires can_change_info administrator right in the channel if it is specified
///
/// # Arguments
///
/// * `chat_id` - Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages member right in the supergroup)
/// * `discussion_chat_id` - Identifier of a new channel's discussion group. Use 0 to remove the discussion group. Use the method getSuitableDiscussionChats to find all suitable groups.
/// Basic group chats must be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable must be used first to change that
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_discussion_group(chat_id: i64, discussion_chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDiscussionGroup",
        "chat_id": chat_id,
        "discussion_chat_id": discussion_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `location` - New location for the chat; must be valid and not null
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_location(chat_id: i64, location: crate::types::ChatLocation, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatLocation",
        "chat_id": chat_id,
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `slow_mode_delay` - New slow mode delay for the chat, in seconds; must be one of 0, 5, 10, 30, 60, 300, 900, 3600
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_slow_mode_delay(chat_id: i64, slow_mode_delay: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatSlowModeDelay",
        "chat_id": chat_id,
        "slow_mode_delay": slow_mode_delay,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds the current user as a new member to a chat. Private and secret chats can't be joined using this method
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatJoinResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn join_chat(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatJoinResult, crate::types::Error> {
    let request = json!({
        "@type": "joinChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes the current user from chat members. Private and secret chats can't be left using this method
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
pub async fn leave_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "leaveChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a new member to a chat; requires can_invite_users member right. Members can't be added to private or secret chats. Returns information about members that weren't added
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user
/// * `forward_limit` - The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels, or if the added user is a bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FailedToAddMembers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_member(chat_id: i64, user_id: i64, forward_limit: i32, client_id: i32) -> Result<crate::enums::FailedToAddMembers, crate::types::Error> {
    let request = json!({
        "@type": "addChatMember",
        "chat_id": chat_id,
        "user_id": user_id,
        "forward_limit": forward_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds multiple new members to a chat; requires can_invite_users member right. Currently, this method is available only in supergroups and channels.
/// This method can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Returns information about members that weren't added
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `user_ids` - Identifiers of the users to be added to the chat. The maximum number of added users is 20 for supergroups and 100 for channels
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FailedToAddMembers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_members(chat_id: i64, user_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::FailedToAddMembers, crate::types::Error> {
    let request = json!({
        "@type": "addChatMembers",
        "chat_id": chat_id,
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the status of a chat member; requires can_invite_users member right to add a chat member, can_promote_members administrator right to change administrator rights of the member,
/// and can_restrict_members administrator right to change restrictions of a user. This function is currently not suitable for transferring chat ownership; use transferChatOwnership instead.
/// Use addChatMember or banChatMember if some additional parameters need to be passed
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier. Chats can be only banned and unbanned in supergroups and channels
/// * `status` - The new status of the member in the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_member_status(chat_id: i64, member_id: crate::enums::MessageSender, status: crate::enums::ChatMemberStatus, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMemberStatus",
        "chat_id": chat_id,
        "member_id": member_id,
        "status": status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the tag or custom title of a chat member; requires can_manage_tags administrator right to change tag of other users; for basic groups and supergroups only
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user whose tag is changed. Chats can't have member tags
/// * `tag` - The new tag of the member in the chat; 0-16 characters without emoji
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_member_tag(chat_id: i64, user_id: i64, tag: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMemberTag",
        "chat_id": chat_id,
        "user_id": user_id,
        "tag": tag,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Bans a member in a chat; requires can_restrict_members administrator right. Members can't be banned in private or secret chats. In supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier
/// * `banned_until_date` - Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Ignored in basic groups and if a chat is banned
/// * `revoke_messages` - Pass true to delete all messages in the chat for the user who is being removed. Always true for supergroups and channels
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn ban_chat_member(chat_id: i64, member_id: crate::enums::MessageSender, banned_until_date: i32, revoke_messages: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "banChatMember",
        "chat_id": chat_id,
        "member_id": member_id,
        "banned_until_date": banned_until_date,
        "revoke_messages": revoke_messages,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the owner of a chat; for basic groups, supergroups and channel chats only; requires owner privileges in the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn transfer_chat_ownership(chat_id: i64, user_id: i64, password: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "transferChatOwnership",
        "chat_id": chat_id,
        "user_id": user_id,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the user who will become the owner of the chat after 7 days if the current user does not return to the supergroup or channel during that period or immediately for basic groups;
/// requires owner privileges in the chat. Available only for basic groups, supergroups, and channel chats
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_owner_after_leaving(chat_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getChatOwnerAfterLeaving",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a single member of a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `member_id` - Member identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatMember)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_member(chat_id: i64, member_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::ChatMember, crate::types::Error> {
    let request = json!({
        "@type": "getChatMember",
        "chat_id": chat_id,
        "member_id": member_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for a specified query in the first name, last name and usernames of the members of a specified chat. Requires administrator rights if the chat is a channel
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `query` - Query to search for
/// * `limit` - The maximum number of users to be returned; up to 200
/// * `filter` - The type of users to search for; pass null to search among all chat members
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatMembers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_members(chat_id: i64, query: String, limit: i32, filter: Option<crate::enums::ChatMembersFilter>, client_id: i32) -> Result<crate::enums::ChatMembers, crate::types::Error> {
    let request = json!({
        "@type": "searchChatMembers",
        "chat_id": chat_id,
        "query": query,
        "limit": limit,
        "filter": filter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a list of administrators of the chat with their custom titles
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatAdministrators)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_administrators(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatAdministrators, crate::types::Error> {
    let request = json!({
        "@type": "getChatAdministrators",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the pinned state of a chat. There can be up to getOption("pinned_chat_count_max")/getOption("pinned_archived_chat_count_max") pinned non-secret chats and the same number of secret chats in the main/archive chat list. The limit can be increased with Telegram Premium
///
/// # Arguments
///
/// * `chat_list` - Chat list in which to change the pinned state of the chat
/// * `chat_id` - Chat identifier
/// * `is_pinned` - Pass true to pin the chat; pass false to unpin it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_pinned(chat_list: crate::enums::ChatList, chat_id: i64, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatIsPinned",
        "chat_list": chat_list,
        "chat_id": chat_id,
        "is_pinned": is_pinned,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the order of pinned chats
///
/// # Arguments
///
/// * `chat_list` - Chat list in which to change the order of pinned chats
/// * `chat_ids` - The new list of pinned chats
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_chats(chat_list: crate::enums::ChatList, chat_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPinnedChats",
        "chat_list": chat_list,
        "chat_ids": chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Traverses all chats in a chat list and marks all messages in the chats as read
///
/// # Arguments
///
/// * `chat_list` - Chat list in which to mark all chats as read
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_chat_list(chat_list: crate::enums::ChatList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readChatList",
        "chat_list": chat_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Replaces current primary invite link for a chat with a new primary invite link. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn replace_primary_chat_invite_link(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "replacePrimaryChatInviteLink",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `name` - Invite link name; 0-32 characters
/// * `expiration_date` - Point in time (Unix timestamp) when the link will expire; pass 0 if never
/// * `member_limit` - The maximum number of chat members that can join the chat via the link simultaneously; 0-99999; pass 0 if not limited
/// * `creates_join_request` - Pass true if users joining the chat via the link need to be approved by chat administrators. In this case, member_limit must be 0
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_invite_link(chat_id: i64, name: String, expiration_date: i32, member_limit: i32, creates_join_request: bool, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatInviteLink",
        "chat_id": chat_id,
        "name": name,
        "expiration_date": expiration_date,
        "member_limit": member_limit,
        "creates_join_request": creates_join_request,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new subscription invite link for a channel chat. Requires can_invite_users right in the chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `name` - Invite link name; 0-32 characters
/// * `subscription_pricing` - Information about subscription plan that will be applied to the users joining the chat via the link.
/// Subscription period must be 2592000 in production environment, and 60 or 300 if Telegram test environment is used
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_subscription_invite_link(chat_id: i64, name: String, subscription_pricing: crate::types::StarSubscriptionPricing, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatSubscriptionInviteLink",
        "chat_id": chat_id,
        "name": name,
        "subscription_pricing": subscription_pricing,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits a non-primary invite link for a chat. Available in basic groups, supergroups, and channels.
/// If the link creates a subscription, then expiration_date, member_limit and creates_join_request must not be used.
/// Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to be edited
/// * `name` - Invite link name; 0-32 characters
/// * `expiration_date` - Point in time (Unix timestamp) when the link will expire; pass 0 if never
/// * `member_limit` - The maximum number of chat members that can join the chat via the link simultaneously; 0-99999; pass 0 if not limited
/// * `creates_join_request` - Pass true if users joining the chat via the link need to be approved by chat administrators. In this case, member_limit must be 0
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_invite_link(chat_id: i64, invite_link: String, name: String, expiration_date: i32, member_limit: i32, creates_join_request: bool, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "editChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "name": name,
        "expiration_date": expiration_date,
        "member_limit": member_limit,
        "creates_join_request": creates_join_request,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits a subscription invite link for a channel chat. Requires can_invite_users right in the chat for own links and owner privileges for other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to be edited
/// * `name` - Invite link name; 0-32 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_subscription_invite_link(chat_id: i64, invite_link: String, name: String, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "editChatSubscriptionInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about an invite link. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to get
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_link(chat_id: i64, invite_link: String, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "getChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of chat administrators with number of their invite links. Requires owner privileges in the chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLinkCounts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_link_counts(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatInviteLinkCounts, crate::types::Error> {
    let request = json!({
        "@type": "getChatInviteLinkCounts",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns invite links for a chat created by specified administrator. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `creator_user_id` - User identifier of a chat administrator. Must be an identifier of the current user for non-owner
/// * `is_revoked` - Pass true if revoked links need to be returned instead of active or expired
/// * `offset_date` - Creation date of an invite link starting after which to return invite links; use 0 to get results from the beginning
/// * `offset_invite_link` - Invite link starting after which to return invite links; use empty string to get results from the beginning
/// * `limit` - The maximum number of invite links to return; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLinks)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_links(chat_id: i64, creator_user_id: i64, is_revoked: bool, offset_date: i32, offset_invite_link: String, limit: i32, client_id: i32) -> Result<crate::enums::ChatInviteLinks, crate::types::Error> {
    let request = json!({
        "@type": "getChatInviteLinks",
        "chat_id": chat_id,
        "creator_user_id": creator_user_id,
        "is_revoked": is_revoked,
        "offset_date": offset_date,
        "offset_invite_link": offset_invite_link,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns chat members joined a chat via an invite link. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to return chat members
/// * `only_with_expired_subscription` - Pass true if the link is a subscription link and only members with expired subscription must be returned
/// * `offset_member` - A chat member from which to return next chat members; pass null to get results from the beginning
/// * `limit` - The maximum number of chat members to return; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLinkMembers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_invite_link_members(chat_id: i64, invite_link: String, only_with_expired_subscription: bool, offset_member: Option<crate::types::ChatInviteLinkMember>, limit: i32, client_id: i32) -> Result<crate::enums::ChatInviteLinkMembers, crate::types::Error> {
    let request = json!({
        "@type": "getChatInviteLinkMembers",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "only_with_expired_subscription": only_with_expired_subscription,
        "offset_member": offset_member,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Revokes invite link for a chat. Available in basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links.
/// If a primary link is revoked, then additionally to the revoked link returns new primary link
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to be revoked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLinks)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn revoke_chat_invite_link(chat_id: i64, invite_link: String, client_id: i32) -> Result<crate::enums::ChatInviteLinks, crate::types::Error> {
    let request = json!({
        "@type": "revokeChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes revoked chat invite links. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link to revoke
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_revoked_chat_invite_link(chat_id: i64, invite_link: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteRevokedChatInviteLink",
        "chat_id": chat_id,
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all revoked chat invite links created by a given chat administrator. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `creator_user_id` - User identifier of a chat administrator, which links will be deleted. Must be an identifier of the current user for non-owner
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_revoked_chat_invite_links(chat_id: i64, creator_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteAllRevokedChatInviteLinks",
        "chat_id": chat_id,
        "creator_user_id": creator_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks the validity of an invite link for a chat and returns information about the corresponding chat
///
/// # Arguments
///
/// * `invite_link` - Invite link to be checked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatInviteLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_chat_invite_link(invite_link: String, client_id: i32) -> Result<crate::enums::ChatInviteLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "checkChatInviteLink",
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Uses an invite link to add the current user to the chat if possible
///
/// # Arguments
///
/// * `invite_link` - Invite link to use
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatJoinResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn join_chat_by_invite_link(invite_link: String, client_id: i32) -> Result<crate::enums::ChatJoinResult, crate::types::Error> {
    let request = json!({
        "@type": "joinChatByInviteLink",
        "invite_link": invite_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns pending join requests in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to return join requests. If empty, all join requests will be returned. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// * `query` - A query to search for in the first names, last names and usernames of the users to return
/// * `offset_request` - A chat join request from which to return next requests; pass null to get results from the beginning
/// * `limit` - The maximum number of requests to join the chat to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatJoinRequests)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_join_requests(chat_id: i64, invite_link: String, query: String, offset_request: Option<crate::types::ChatJoinRequest>, limit: i32, client_id: i32) -> Result<crate::enums::ChatJoinRequests, crate::types::Error> {
    let request = json!({
        "@type": "getChatJoinRequests",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "query": query,
        "offset_request": offset_request,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Handles a pending join request in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user who sent the request
/// * `approve` - Pass true to approve the request; pass false to decline it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_join_request(chat_id: i64, user_id: i64, approve: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processChatJoinRequest",
        "chat_id": chat_id,
        "user_id": user_id,
        "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Handles all pending join requests for a given link in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `invite_link` - Invite link for which to process join requests. If empty, all join requests will be processed. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
/// * `approve` - Pass true to approve all requests; pass false to decline them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_join_requests(chat_id: i64, invite_link: String, approve: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processChatJoinRequests",
        "chat_id": chat_id,
        "invite_link": invite_link,
        "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the personal chat of the current user
///
/// # Arguments
///
/// * `chat_id` - Identifier of the new personal chat; pass 0 to remove the chat. Use getSuitablePersonalChats to get suitable chats
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_personal_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPersonalChat",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets default administrator rights for adding the bot to channel chats; for bots only
///
/// # Arguments
///
/// * `default_channel_administrator_rights` - Default administrator rights for adding the bot to channels; pass null to remove default rights
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_default_channel_administrator_rights(default_channel_administrator_rights: Option<crate::types::ChatAdministratorRights>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDefaultChannelAdministratorRights",
        "default_channel_administrator_rights": default_channel_administrator_rights,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a session can accept incoming secret chats
///
/// # Arguments
///
/// * `session_id` - Session identifier
/// * `can_accept_secret_chats` - Pass true to allow accepting secret chats by the session; pass false otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_session_can_accept_secret_chats(session_id: i64, can_accept_secret_chats: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSessionCanAcceptSecretChats",
        "session_id": session_id,
        "can_accept_secret_chats": can_accept_secret_chats,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the editable username of a supergroup or channel, requires owner privileges in the supergroup or channel
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `username` - New value of the username. Use an empty string to remove the username. The username can't be completely removed if there is another active or disabled username
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_username(supergroup_id: i64, username: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupUsername",
        "supergroup_id": supergroup_id,
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes active state for a username of a supergroup or channel, requires owner privileges in the supergroup or channel. The editable username can't be disabled.
/// May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `username` - The username to change
/// * `is_active` - Pass true to activate the username; pass false to disable it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_username_is_active(supergroup_id: i64, username: String, is_active: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupUsernameIsActive",
        "supergroup_id": supergroup_id,
        "username": username,
        "is_active": is_active,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Disables all active non-editable usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn disable_all_supergroup_usernames(supergroup_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disableAllSupergroupUsernames",
        "supergroup_id": supergroup_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes order of active usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_supergroup_active_usernames(supergroup_id: i64, usernames: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderSupergroupActiveUsernames",
        "supergroup_id": supergroup_id,
        "usernames": usernames,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether all users directly joining the supergroup need to be approved by supergroup administrators; requires can_restrict_members administrator right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup that isn't a broadcast group and isn't a channel direct message group
/// * `join_by_request` - New value of join_by_request
/// * `guard_bot_user_id` - Identifier of the bot which will be the guard bot in the group; pass 0 if none; ignored if join_by_request == false.
/// The bot must have administrator privileges and can_invite_users right in the supergroup chat, and must have userTypeBot.is_guard == true
/// * `apply_to_invite_links` - Pass true to apply the change to the existing invite links, including primary links
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_join_by_request(supergroup_id: i64, join_by_request: bool, guard_bot_user_id: i64, apply_to_invite_links: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupJoinByRequest",
        "supergroup_id": supergroup_id,
        "join_by_request": join_by_request,
        "guard_bot_user_id": guard_bot_user_id,
        "apply_to_invite_links": apply_to_invite_links,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether messages are automatically translated in the channel chat; requires can_change_info administrator right in the channel.
/// The chat must have at least chatBoostFeatures.min_automatic_translation_boost_level boost level to enable automatic translation
///
/// # Arguments
///
/// * `supergroup_id` - The identifier of the channel
/// * `has_automatic_translation` - The new value of has_automatic_translation
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_automatic_translation(supergroup_id: i64, has_automatic_translation: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupHasAutomaticTranslation",
        "supergroup_id": supergroup_id,
        "has_automatic_translation": has_automatic_translation,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers. Can be called only if supergroupFullInfo.can_hide_members == true
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `has_hidden_members` - New value of has_hidden_members
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_hidden_members(supergroup_id: i64, has_hidden_members: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupHasHiddenMembers",
        "supergroup_id": supergroup_id,
        "has_hidden_members": has_hidden_members,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether aggressive anti-spam checks are enabled in the supergroup. Can be called only if supergroupFullInfo.can_toggle_aggressive_anti_spam == true
///
/// # Arguments
///
/// * `supergroup_id` - The identifier of the supergroup, which isn't a broadcast group
/// * `has_aggressive_anti_spam_enabled` - The new value of has_aggressive_anti_spam_enabled
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_aggressive_anti_spam_enabled(supergroup_id: i64, has_aggressive_anti_spam_enabled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupHasAggressiveAntiSpamEnabled",
        "supergroup_id": supergroup_id,
        "has_aggressive_anti_spam_enabled": has_aggressive_anti_spam_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Upgrades supergroup to a broadcast group; requires owner privileges in the supergroup
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_broadcast_group(supergroup_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsBroadcastGroup",
        "supergroup_id": supergroup_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports messages in a supergroup as spam; requires administrator rights in the supergroup
///
/// # Arguments
///
/// * `supergroup_id` - Supergroup identifier
/// * `message_ids` - Identifiers of messages to report. Use messageProperties.can_report_supergroup_spam to check whether the message can be reported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_supergroup_spam(supergroup_id: i64, message_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportSupergroupSpam",
        "supergroup_id": supergroup_id,
        "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports a false deletion of a message by aggressive anti-spam checks; requires administrator rights in the supergroup. Can be called only for messages from chatEventMessageDeleted with can_report_anti_spam_false_positive == true
///
/// # Arguments
///
/// * `supergroup_id` - Supergroup identifier
/// * `message_id` - Identifier of the erroneously deleted message from chatEventMessageDeleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_supergroup_anti_spam_false_positive(supergroup_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportSupergroupAntiSpamFalsePositive",
        "supergroup_id": supergroup_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about members or banned users in a supergroup or channel. Can be used only if supergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `filter` - The type of users to return; pass null to use supergroupMembersFilterRecent
/// * `offset` - Number of users to skip
/// * `limit` - The maximum number of users to be returned; up to 200
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatMembers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_supergroup_members(supergroup_id: i64, filter: Option<crate::enums::SupergroupMembersFilter>, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::ChatMembers, crate::types::Error> {
    let request = json!({
        "@type": "getSupergroupMembers",
        "supergroup_id": supergroup_id,
        "filter": filter,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Closes a secret chat, effectively transferring its state to secretChatStateClosed
///
/// # Arguments
///
/// * `secret_chat_id` - Secret chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn close_secret_chat(secret_chat_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeSecretChat",
        "secret_chat_id": secret_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only in supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i.e., in order of decreasing event_id)
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `query` - Search query by which to filter events
/// * `from_event_id` - Identifier of an event from which to return results. Use 0 to get results from the latest events
/// * `limit` - The maximum number of events to return; up to 100
/// * `filters` - The types of events to return; pass null to get chat events of all types
/// * `user_ids` - User identifiers by which to filter events. By default, events relating to all users will be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatEvents)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_event_log(chat_id: i64, query: String, from_event_id: i64, limit: i32, filters: Option<crate::types::ChatEventLogFilters>, user_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::ChatEvents, crate::types::Error> {
    let request = json!({
        "@type": "getChatEventLog",
        "chat_id": chat_id,
        "query": query,
        "from_event_id": from_event_id,
        "limit": limit,
        "filters": filters,
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes privacy settings for new chat creation; can be used only if getOption("can_set_new_chat_privacy_settings")
///
/// # Arguments
///
/// * `settings` - New settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_new_chat_privacy_settings(settings: crate::types::NewChatPrivacySettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setNewChatPrivacySettings",
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns privacy settings for new chat creation
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::NewChatPrivacySettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_new_chat_privacy_settings(client_id: i32) -> Result<crate::enums::NewChatPrivacySettings, crate::types::Error> {
    let request = json!({
        "@type": "getNewChatPrivacySettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a chat action bar without any other action
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
pub async fn remove_chat_action_bar(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeChatActionBar",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if chat.can_be_reported
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `message_ids` - Identifiers of reported messages. Use messageProperties.can_report_chat to check whether the message can be reported
/// * `text` - Additional report details if asked by the server; 0-1024 characters; leave empty for the initial request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReportChatResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_chat(chat_id: i64, option_id: String, message_ids: Vec<i64>, text: String, client_id: i32) -> Result<crate::enums::ReportChatResult, crate::types::Error> {
    let request = json!({
        "@type": "reportChat",
        "chat_id": chat_id,
        "option_id": option_id,
        "message_ids": message_ids,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns detailed statistics about a chat. Currently, this method can be used only for supergroups and channels. Can be used only if supergroupFullInfo.can_get_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_statistics(chat_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::ChatStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getChatStatistics",
        "chat_id": chat_id,
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

