//!
//! TDLib `payment` domain functions.
//!
//! Types, enums, and functions for invoices, payments, Telegram Stars, and monetized features.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns the total number of Telegram Stars received by the channel chat for direct messages from the given topic
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat administered by the current user
/// * `topic_id` - Identifier of the topic
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarCount)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic_revenue(chat_id: i64, topic_id: i64, client_id: i32) -> Result<crate::enums::StarCount, crate::types::Error> {
    let request = json!({
        "@type": "getDirectMessagesChatTopicRevenue",
        "chat_id": chat_id,
        "topic_id": topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the Telegram Star amount owned by a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarAmount)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_business_account_star_amount(business_connection_id: String, client_id: i32) -> Result<crate::enums::StarAmount, crate::types::Error> {
    let request = json!({
        "@type": "getBusinessAccountStarAmount",
        "business_connection_id": business_connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Transfers Telegram Stars from the business account to the business bot; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `star_count` - Number of Telegram Stars to transfer
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn transfer_business_account_stars(business_connection_id: String, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "transferBusinessAccountStars",
        "business_connection_id": business_connection_id,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the result of a shipping query; for bots only
///
/// # Arguments
///
/// * `shipping_query_id` - Identifier of the shipping query
/// * `shipping_options` - Available shipping options
/// * `error_message` - An error message, empty on success
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_shipping_query(shipping_query_id: i64, shipping_options: Vec<crate::types::ShippingOption>, error_message: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerShippingQuery",
        "shipping_query_id": shipping_query_id,
        "shipping_options": shipping_options,
        "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the result of a pre-checkout query; for bots only
///
/// # Arguments
///
/// * `pre_checkout_query_id` - Identifier of the pre-checkout query
/// * `error_message` - An error message, empty on success
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_pre_checkout_query(pre_checkout_query_id: i64, error_message: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerPreCheckoutQuery",
        "pre_checkout_query_id": pre_checkout_query_id,
        "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the minimum number of Telegram Stars that must be paid by general participant for each sent message to a live story call. Requires groupCall.can_be_managed right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier; must be an identifier of a live story call
/// * `paid_message_star_count` - The new minimum number of Telegram Stars; 0-getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_paid_message_star_count(group_call_id: i32, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallPaidMessageStarCount",
        "group_call_id": group_call_id,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns an invoice payment form. This method must be called when the user presses inline button of the type inlineKeyboardButtonTypeBuy, or wants to buy access to media in a messagePaidMedia message
///
/// # Arguments
///
/// * `input_invoice` - The invoice
/// * `theme` - Preferred payment form theme; pass null to use the default theme
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PaymentForm)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_payment_form(input_invoice: crate::enums::InputInvoice, theme: Option<crate::types::ThemeParameters>, client_id: i32) -> Result<crate::enums::PaymentForm, crate::types::Error> {
    let request = json!({
        "@type": "getPaymentForm",
        "input_invoice": input_invoice,
        "theme": theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Validates the order information provided by a user and returns the available shipping options for a flexible invoice
///
/// # Arguments
///
/// * `input_invoice` - The invoice
/// * `order_info` - The order information, provided by the user; pass null if empty
/// * `allow_save` - Pass true to save the order information
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ValidatedOrderInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn validate_order_info(input_invoice: crate::enums::InputInvoice, order_info: Option<crate::types::OrderInfo>, allow_save: bool, client_id: i32) -> Result<crate::enums::ValidatedOrderInfo, crate::types::Error> {
    let request = json!({
        "@type": "validateOrderInfo",
        "input_invoice": input_invoice,
        "order_info": order_info,
        "allow_save": allow_save,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a filled-out payment form to the bot for final verification
///
/// # Arguments
///
/// * `input_invoice` - The invoice
/// * `payment_form_id` - Payment form identifier returned by getPaymentForm
/// * `order_info_id` - Identifier returned by validateOrderInfo, or an empty string
/// * `shipping_option_id` - Identifier of a chosen shipping option, if applicable
/// * `credentials` - The credentials chosen by user for payment; pass null for a payment in Telegram Stars
/// * `tip_amount` - Chosen by the user amount of tip in the smallest units of the currency
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PaymentResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_payment_form(input_invoice: crate::enums::InputInvoice, payment_form_id: i64, order_info_id: String, shipping_option_id: String, credentials: Option<crate::enums::InputCredentials>, tip_amount: i64, client_id: i32) -> Result<crate::enums::PaymentResult, crate::types::Error> {
    let request = json!({
        "@type": "sendPaymentForm",
        "input_invoice": input_invoice,
        "payment_form_id": payment_form_id,
        "order_info_id": order_info_id,
        "shipping_option_id": shipping_option_id,
        "credentials": credentials,
        "tip_amount": tip_amount,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a successful payment
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the messagePaymentSuccessful message
/// * `message_id` - Message identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PaymentReceipt)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_payment_receipt(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::PaymentReceipt, crate::types::Error> {
    let request = json!({
        "@type": "getPaymentReceipt",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns saved order information. Returns a 404 error if there is no saved order information
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::OrderInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_order_info(client_id: i32) -> Result<crate::enums::OrderInfo, crate::types::Error> {
    let request = json!({
        "@type": "getSavedOrderInfo",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes saved order information
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
pub async fn delete_saved_order_info(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedOrderInfo",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Creates a link for the given invoice; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `invoice` - Information about the invoice of the type inputMessageInvoice
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_invoice_link(business_connection_id: String, invoice: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "createInvoiceLink",
        "business_connection_id": business_connection_id,
        "invoice": invoice,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Refunds a previously done payment in Telegram Stars; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user who did the payment
/// * `telegram_payment_charge_id` - Telegram payment identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn refund_star_payment(user_id: i64, telegram_payment_charge_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "refundStarPayment",
        "user_id": user_id,
        "telegram_payment_charge_id": telegram_payment_charge_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the total number of Telegram Stars received by the current user for paid messages from the given user
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarCount)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_paid_message_revenue(user_id: i64, client_id: i32) -> Result<crate::enums::StarCount, crate::types::Error> {
    let request = json!({
        "@type": "getPaidMessageRevenue",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the Telegram Star amount that must be paid to send a message to a supergroup chat; requires can_restrict_members administrator right and supergroupFullInfo.can_enable_paid_messages
///
/// # Arguments
///
/// * `chat_id` - Identifier of the supergroup chat
/// * `paid_message_star_count` - The new number of Telegram Stars that must be paid for each message that is sent to the supergroup chat unless the sender is an administrator of the chat; 0-getOption("paid_message_star_count_max").
/// The supergroup will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_paid_message_star_count(chat_id: i64, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPaidMessageStarCount",
        "chat_id": chat_id,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns detailed revenue statistics about a chat. Currently, this method can be used only
/// for channels if supergroupFullInfo.can_get_revenue_statistics == true or bots if userFullInfo.bot_info.can_get_revenue_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatRevenueStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_revenue_statistics(chat_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::ChatRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getChatRevenueStatistics",
        "chat_id": chat_id,
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a URL for chat revenue withdrawal; requires owner privileges in the channel chat or the bot. Currently, this method can be used only
/// if getOption("can_withdraw_chat_revenue") for channels with supergroupFullInfo.can_get_revenue_statistics == true or bots with userFullInfo.bot_info.can_get_revenue_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_revenue_withdrawal_url(chat_id: i64, password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getChatRevenueWithdrawalUrl",
        "chat_id": chat_id,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of revenue transactions for a chat. Currently, this method can be used only
/// for channels if supergroupFullInfo.can_get_revenue_statistics == true or bots if userFullInfo.bot_info.can_get_revenue_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatRevenueTransactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_revenue_transactions(chat_id: i64, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::ChatRevenueTransactions, crate::types::Error> {
    let request = json!({
        "@type": "getChatRevenueTransactions",
        "chat_id": chat_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns detailed Telegram Star revenue statistics
///
/// # Arguments
///
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of the current user, an owned bot, or a supergroup or a channel chat with supergroupFullInfo.can_get_star_revenue_statistics == true
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarRevenueStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_revenue_statistics(owner_id: crate::enums::MessageSender, is_dark: bool, client_id: i32) -> Result<crate::enums::StarRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getStarRevenueStatistics",
        "owner_id": owner_id,
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a URL for Telegram Star withdrawal
///
/// # Arguments
///
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of the current user, an owned bot, or an owned supergroup or channel chat
/// * `star_count` - The number of Telegram Stars to withdraw; must be between getOption("star_withdrawal_count_min") and getOption("star_withdrawal_count_max")
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_withdrawal_url(owner_id: crate::enums::MessageSender, star_count: i64, password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getStarWithdrawalUrl",
        "owner_id": owner_id,
        "star_count": star_count,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a URL for a Telegram Ad platform account that can be used to set up advertisements for the chat paid in the owned Telegram Stars
///
/// # Arguments
///
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of an owned bot, or identifier of an owned channel chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_ad_account_url(owner_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getStarAdAccountUrl",
        "owner_id": owner_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns detailed TON Gram revenue statistics of the current user
///
/// # Arguments
///
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GramRevenueStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gram_revenue_statistics(is_dark: bool, client_id: i32) -> Result<crate::enums::GramRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getGramRevenueStatistics",
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available options for gifting Telegram Premium to a user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumGiftPaymentOptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_gift_payment_options(client_id: i32) -> Result<crate::enums::PremiumGiftPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumGiftPaymentOptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
///
/// # Arguments
///
/// * `boosted_chat_id` - Identifier of the supergroup or channel chat, which will be automatically boosted by receivers of the gift codes and which is administered by the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PremiumGiveawayPaymentOptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_giveaway_payment_options(boosted_chat_id: i64, client_id: i32) -> Result<crate::enums::PremiumGiveawayPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumGiveawayPaymentOptions",
        "boosted_chat_id": boosted_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Allows to buy a Telegram Premium subscription for another user with payment in Telegram Stars; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user who will receive Telegram Premium
/// * `star_count` - The number of Telegram Stars to pay for subscription
/// * `month_count` - Number of months the Telegram Premium subscription will be active for the user
/// * `text` - Text to show to the user receiving Telegram Premium; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn gift_premium_with_stars(user_id: i64, star_count: i64, month_count: i32, text: crate::types::FormattedText, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "giftPremiumWithStars",
        "user_id": user_id,
        "star_count": star_count,
        "month_count": month_count,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns available options for Telegram Stars purchase
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarPaymentOptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_payment_options(client_id: i32) -> Result<crate::enums::StarPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getStarPaymentOptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available options for Telegram Stars gifting
///
/// # Arguments
///
/// * `user_id` - Identifier of the user who will receive Telegram Stars; pass 0 to get options for an unspecified user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarPaymentOptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_gift_payment_options(user_id: i64, client_id: i32) -> Result<crate::enums::StarPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getStarGiftPaymentOptions",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns available options for Telegram Star giveaway creation
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarGiveawayPaymentOptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_giveaway_payment_options(client_id: i32) -> Result<crate::enums::StarGiveawayPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getStarGiveawayPaymentOptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of Telegram Star transactions for the specified owner
///
/// # Arguments
///
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be the identifier of the current user, identifier of an owned bot,
/// or identifier of a supergroup or a channel chat with supergroupFullInfo.can_get_star_revenue_statistics == true
/// * `subscription_id` - If non-empty, only transactions related to the Star Subscription will be returned
/// * `direction` - Direction of the transactions to receive; pass null to get all transactions
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarTransactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_transactions(owner_id: crate::enums::MessageSender, subscription_id: String, direction: Option<crate::enums::TransactionDirection>, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::StarTransactions, crate::types::Error> {
    let request = json!({
        "@type": "getStarTransactions",
        "owner_id": owner_id,
        "subscription_id": subscription_id,
        "direction": direction,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of Telegram Star subscriptions for the current user
///
/// # Arguments
///
/// * `only_expiring` - Pass true to receive only expiring subscriptions for which there aren't enough Telegram Stars to extend
/// * `offset` - Offset of the first subscription to return as received from the previous request; use empty string to get the first chunk of results
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StarSubscriptions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_star_subscriptions(only_expiring: bool, offset: String, client_id: i32) -> Result<crate::enums::StarSubscriptions, crate::types::Error> {
    let request = json!({
        "@type": "getStarSubscriptions",
        "only_expiring": only_expiring,
        "offset": offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Cancels or re-enables Telegram Star subscription
///
/// # Arguments
///
/// * `subscription_id` - Identifier of the subscription to change
/// * `is_canceled` - New value of is_canceled
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_star_subscription(subscription_id: String, is_canceled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editStarSubscription",
        "subscription_id": subscription_id,
        "is_canceled": is_canceled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Cancels or re-enables Telegram Star subscription for a user; for bots only
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `telegram_payment_charge_id` - Telegram payment identifier of the subscription
/// * `is_canceled` - Pass true to cancel the subscription; pass false to allow the user to enable it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_user_star_subscription(user_id: i64, telegram_payment_charge_id: String, is_canceled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editUserStarSubscription",
        "user_id": user_id,
        "telegram_payment_charge_id": telegram_payment_charge_id,
        "is_canceled": is_canceled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reuses an active Telegram Star subscription to a channel chat and joins the chat again
///
/// # Arguments
///
/// * `subscription_id` - Identifier of the subscription
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reuse_star_subscription(subscription_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reuseStarSubscription",
        "subscription_id": subscription_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

