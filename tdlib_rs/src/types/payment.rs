//!
//! TDLib `payment` domain types.
//!
//! Types, enums, and functions for invoices, payments, Telegram Stars, and monetized features.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The post was refunded, because the payment for the post was refunded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostRefundReasonPaymentRefunded {
}

/// Describes a subscription to a channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionTypeChannel {
    /// True, if the subscription is active and the user can use the method reuseStarSubscription to join the subscribed chat again
    pub can_reuse: bool,
    /// The invite link that can be used to renew the subscription if it has expired; may be empty if the link isn't available anymore
    pub invite_link: String,
}

/// Describes a subscription in a bot or a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionTypeBot {
    /// True, if the subscription was canceled by the bot and can't be extended
    pub is_canceled_by_bot: bool,
    /// Subscription invoice title
    pub title: String,
    /// Subscription invoice photo
    pub photo: crate::types::Photo,
    /// The link to the subscription invoice
    pub invoice_link: String,
}

/// Describes subscription plan paid in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionPricing {
    /// The number of seconds between consecutive Telegram Star debiting
    pub period: i32,
    /// The Telegram Star amount that must be paid for each period
    pub star_count: i64,
}

/// Contains information about subscription to a channel chat, a bot, or a business account that was paid in Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarSubscription {
    /// Unique identifier of the subscription
    pub id: String,
    /// Identifier of the chat that is subscribed
    pub chat_id: i64,
    /// Point in time (Unix timestamp) when the subscription will expire or expired
    pub expiration_date: i32,
    /// True, if the subscription was canceled
    pub is_canceled: bool,
    /// True, if the subscription expires soon and there aren't enough Telegram Stars on the user's balance to extend it
    pub is_expiring: bool,
    /// The subscription plan
    pub pricing: crate::types::StarSubscriptionPricing,
    /// Type of the subscription
    #[serde(rename = "type")]
    pub r#type: crate::enums::StarSubscriptionType,
}

/// Represents a list of Telegram Star subscriptions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptions {
    /// The amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// List of subscriptions for Telegram Stars
    pub subscriptions: Vec<crate::types::StarSubscription>,
    /// The number of Telegram Stars required to buy to extend subscriptions expiring soon
    pub required_star_count: i64,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// The affiliate programs must be sorted by the expected revenue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramSortOrderRevenue {
}

/// Describes an option for buying Telegram Premium to a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumPaymentOption {
    /// ISO 4217 currency code for Telegram Premium subscription payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// The discount associated with this option, as a percentage
    pub discount_percentage: i32,
    /// Number of months the Telegram Premium subscription will be active. Use getPremiumInfoSticker to get the sticker to be used as representation of the Telegram Premium subscription
    pub month_count: i32,
    /// Identifier of the store product associated with the option
    pub store_product_id: String,
    /// An internal link to be opened for buying Telegram Premium to the user if store payment isn't possible; may be null if direct payment isn't available
    pub payment_link: Option<crate::enums::InternalLinkType>,
}

/// Describes an option for buying or upgrading Telegram Premium for self
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStatePaymentOption {
    /// Information about the payment option
    pub payment_option: crate::types::PremiumPaymentOption,
    /// True, if this is the currently used Telegram Premium subscription option
    pub is_current: bool,
    /// True, if the payment option can be used to upgrade the existing Telegram Premium subscription
    pub is_upgrade: bool,
    /// Identifier of the last in-store transaction for the currently used option
    pub last_transaction_id: String,
}

/// Describes an option for gifting Telegram Premium to a user. Use telegramPaymentPurposePremiumGift for out-of-store payments or payments in Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// The alternative Telegram Star amount to pay; 0 if payment in Telegram Stars is not possible
    pub star_count: i64,
    /// The discount associated with this option, as a percentage
    pub discount_percentage: i32,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// Identifier of the store product associated with the option
    pub store_product_id: String,
    /// A sticker to be shown along with the option; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// Contains a list of options for gifting Telegram Premium to a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftPaymentOptions {
    /// The list of options sorted by Telegram Premium subscription duration
    pub options: Vec<crate::types::PremiumGiftPaymentOption>,
}

/// Describes an option for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members. Use telegramPaymentPurposePremiumGiftCodes or telegramPaymentPurposePremiumGiveaway for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiveawayPaymentOption {
    /// ISO 4217 currency code for Telegram Premium gift code payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of users who will be able to activate the gift codes
    pub winner_count: i32,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// Number of times the store product must be paid
    pub store_product_quantity: i32,
}

/// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiveawayPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::PremiumGiveawayPaymentOption>,
}

/// Describes an option for buying Telegram Stars. Use telegramPaymentPurposeStars for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of Telegram Stars that will be purchased
    pub star_count: i64,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// True, if the option must be shown only in the full list of payment options
    pub is_additional: bool,
}

/// Contains a list of options for buying Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::StarPaymentOption>,
}

/// Describes an option for creating of Telegram Star giveaway. Use telegramPaymentPurposeStarGiveaway for out-of-store payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayPaymentOption {
    /// ISO 4217 currency code for the payment
    pub currency: String,
    /// The amount to pay, in the smallest units of the currency
    pub amount: i64,
    /// Number of Telegram Stars that will be distributed among winners
    pub star_count: i64,
    /// Identifier of the store product associated with the option; may be empty if none
    pub store_product_id: String,
    /// Number of times the chat will be boosted for one year if the option is chosen
    pub yearly_boost_count: i32,
    /// Allowed options for the number of giveaway winners
    pub winner_options: Vec<crate::types::StarGiveawayWinnerOption>,
    /// True, if the option must be chosen by default
    pub is_default: bool,
    /// True, if the option must be shown only in the full list of payment options
    pub is_additional: bool,
}

/// Contains a list of options for creating of Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::StarGiveawayPaymentOption>,
}

/// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotInvoicePurchase {
    /// Identifier of the bot or the business account user who created the invoice
    pub user_id: i64,
    /// Information about the bought product
    pub product_info: crate::types::ProductInfo,
}

/// The transaction is a sale of a product by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotInvoiceSale {
    /// Identifier of the user who bought the product
    pub user_id: i64,
    /// Information about the bought product
    pub product_info: crate::types::ProductInfo,
    /// Invoice payload
    pub invoice_payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}

/// The transaction is a payment for a suggested post; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeSuggestedPostPaymentSend {
    /// Identifier of the channel chat that posted the post
    pub chat_id: i64,
}

/// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeSuggestedPostPaymentReceive {
    /// Identifier of the user who paid for the suggested post
    pub user_id: i64,
}

/// The transaction is a payment for a suggested post
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeSuggestedPostPayment {
    /// Identifier of the channel chat that posted the post
    pub chat_id: i64,
}

/// The giveaway sends Telegram Stars to the winners
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayPrizeStars {
    /// Number of Telegram Stars that will be shared by all winners
    pub star_count: i64,
}

/// The link is a link to an invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeInvoice {
}

/// Product invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Invoice {
    /// ISO 4217 currency code
    pub currency: String,
    /// A list of objects used to calculate the total price of the product
    pub price_parts: Vec<crate::types::LabeledPricePart>,
    /// The number of seconds between consecutive Telegram Star debiting for subscription invoices; 0 if the invoice doesn't create subscription
    pub subscription_period: i32,
    /// The maximum allowed amount of tip in the smallest units of the currency
    pub max_tip_amount: i64,
    /// Suggested amounts of tip in the smallest units of the currency
    pub suggested_tip_amounts: Vec<i64>,
    /// An HTTP URL with terms of service for recurring payments. If non-empty, the invoice payment will result in recurring payments and the user must accept the terms of service before allowed to pay
    pub recurring_payment_terms_of_service_url: String,
    /// An HTTP URL with terms of service for non-recurring payments. If non-empty, then the user must accept the terms of service before allowed to pay
    pub terms_of_service_url: String,
    /// True, if the payment is a test payment
    pub is_test: bool,
    /// True, if the user's name is needed for payment
    pub need_name: bool,
    /// True, if the user's phone number is needed for payment
    pub need_phone_number: bool,
    /// True, if the user's email address is needed for payment
    pub need_email_address: bool,
    /// True, if the user's shipping address is needed for payment
    pub need_shipping_address: bool,
    /// True, if the user's phone number will be sent to the provider
    pub send_phone_number_to_provider: bool,
    /// True, if the user's email address will be sent to the provider
    pub send_email_address_to_provider: bool,
    /// True, if the total price depends on the shipping method
    pub is_flexible: bool,
}

/// One shipping option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// A list of objects used to calculate the total shipping costs
    pub price_parts: Vec<crate::types::LabeledPricePart>,
}

/// Smart Glocal payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderSmartGlocal {
    /// Public payment token
    pub public_token: String,
    /// URL for sending card tokenization requests
    pub tokenize_url: String,
}

/// Stripe payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderStripe {
    /// Stripe API publishable key
    pub publishable_key: String,
    /// True, if the user country must be provided
    pub need_country: bool,
    /// True, if the user ZIP/postal code must be provided
    pub need_postal_code: bool,
    /// True, if the cardholder name must be provided
    pub need_cardholder_name: bool,
}

/// Some other payment provider, for which a web payment form must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderOther {
    /// Payment form URL
    pub url: String,
}

/// Describes an additional payment option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentOption {
    /// Title for the payment option
    pub title: String,
    /// Payment form URL to be opened in a web view
    pub url: String,
}

/// The payment form is for a regular payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeRegular {
    /// Full information about the invoice
    pub invoice: crate::types::Invoice,
    /// User identifier of the payment provider bot
    pub payment_provider_user_id: i64,
    /// Information about the payment provider
    pub payment_provider: crate::enums::PaymentProvider,
    /// The list of additional payment options
    pub additional_payment_options: Vec<crate::types::PaymentOption>,
    /// Saved server-side order information; may be null
    pub saved_order_info: Option<crate::types::OrderInfo>,
    /// The list of saved payment credentials
    pub saved_credentials: Vec<crate::types::SavedCredentials>,
    /// True, if the user can choose to save credentials
    pub can_save_credentials: bool,
    /// True, if the user will be able to save credentials, if sets up a 2-step verification password
    pub need_password: bool,
}

/// The payment form is for a payment in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeStars {
    /// Number of Telegram Stars that will be paid
    pub star_count: i64,
}

/// The payment form is for a payment in Telegram Stars for subscription
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeStarSubscription {
    /// Information about subscription plan
    pub pricing: crate::types::StarSubscriptionPricing,
}

/// Contains information about an invoice payment form
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentForm {
    /// The payment form identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Type of the payment form
    #[serde(rename = "type")]
    pub r#type: crate::enums::PaymentFormType,
    /// User identifier of the seller bot
    pub seller_bot_user_id: i64,
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
}

/// Contains the result of a payment request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentResult {
    /// True, if the payment request was successful; otherwise, the verification_url will be non-empty
    pub success: bool,
    /// URL for additional payment credentials verification
    pub verification_url: String,
}

/// The payment was done using a third-party payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceiptTypeRegular {
    /// User identifier of the payment provider bot
    pub payment_provider_user_id: i64,
    /// Information about the invoice
    pub invoice: crate::types::Invoice,
    /// Order information; may be null
    pub order_info: Option<crate::types::OrderInfo>,
    /// Chosen shipping option; may be null
    pub shipping_option: Option<crate::types::ShippingOption>,
    /// Title of the saved credentials chosen by the buyer
    pub credentials_title: String,
    /// The amount of tip chosen by the buyer in the smallest units of the currency
    pub tip_amount: i64,
}

/// The payment was done using Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceiptTypeStars {
    /// Number of Telegram Stars that were paid
    pub star_count: i64,
    /// Unique identifier of the transaction that can be used to dispute it
    pub transaction_id: String,
}

/// Contains information about a successful payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceipt {
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
    /// Point in time (Unix timestamp) when the payment was made
    pub date: i32,
    /// User identifier of the seller bot
    pub seller_bot_user_id: i64,
    /// Type of the payment receipt
    #[serde(rename = "type")]
    pub r#type: crate::enums::PaymentReceiptType,
}

/// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceMessage {
    /// Chat identifier of the message
    pub chat_id: i64,
    /// Message identifier. Use messageProperties.can_be_paid to check whether the message can be used in the method
    pub message_id: i64,
}

/// An invoice from a link of the type internalLinkTypeInvoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceName {
    /// Name of the invoice
    pub name: String,
}

/// An invoice for a payment toward Telegram; must not be used in the in-store apps
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceTelegram {
    /// Transaction purpose
    pub purpose: crate::enums::TelegramPaymentPurpose,
}

/// A message with an invoice from a bot. Use getInternalLink with internalLinkTypeBotStart to share the invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageInvoice {
    /// Information about the product
    pub product_info: crate::types::ProductInfo,
    /// Currency for the product price
    pub currency: String,
    /// Product total price in the smallest units of the currency
    pub total_amount: i64,
    /// Unique invoice bot start_parameter to be passed to getInternalLink
    pub start_parameter: String,
    /// True, if the invoice is a test invoice
    pub is_test: bool,
    /// True, if the shipping address must be specified
    pub need_shipping_address: bool,
    /// The identifier of the message with the receipt, after the product has been purchased
    pub receipt_message_id: i64,
    /// Extended media attached to the invoice; may be null if none
    pub paid_media: Option<crate::enums::PaidMedia>,
    /// Extended media caption; may be null if none
    pub paid_media_caption: Option<crate::types::FormattedText>,
}

/// A payment has been sent to a bot or a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentSuccessful {
    /// Identifier of the chat, containing the corresponding invoice message
    pub invoice_chat_id: i64,
    /// Identifier of the message with the corresponding invoice; may be 0 or an identifier of a deleted message
    pub invoice_message_id: i64,
    /// Currency for the price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Point in time (Unix timestamp) when the subscription will expire; 0 if unknown or the payment isn't recurring
    pub subscription_until_date: i32,
    /// True, if this is a recurring payment
    pub is_recurring: bool,
    /// True, if this is the first recurring payment
    pub is_first_recurring: bool,
    /// Name of the invoice; may be empty if unknown
    pub invoice_name: String,
}

/// A payment has been received by the bot or the business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentSuccessfulBot {
    /// Currency for price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Point in time (Unix timestamp) when the subscription will expire; 0 if unknown or the payment isn't recurring
    pub subscription_until_date: i32,
    /// True, if this is a recurring payment
    pub is_recurring: bool,
    /// True, if this is the first recurring payment
    pub is_first_recurring: bool,
    /// Invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user; may be empty if not applicable; for bots only
    pub shipping_option_id: String,
    /// Information about the order; may be null; for bots only
    pub order_info: Option<crate::types::OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

/// A payment has been refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePaymentRefunded {
    /// Identifier of the previous owner of the Telegram Stars that refunds them
    pub owner_id: crate::enums::MessageSender,
    /// Currency for the price of the product
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Invoice payload; only for bots
    pub invoice_payload: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

/// Telegram Stars were gifted to a user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedStars {
    /// The identifier of a user who gifted Telegram Stars; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Telegram Stars; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of Telegram Stars that were gifted
    pub star_count: i64,
    /// Identifier of the transaction for Telegram Stars purchase; for receiver only
    pub transaction_id: String,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// Telegram Stars were received by the current user from a giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayPrizeStars {
    /// Number of Telegram Stars that were received
    pub star_count: i64,
    /// Identifier of the transaction for Telegram Stars credit
    pub transaction_id: String,
    /// Identifier of the supergroup or channel chat, which was automatically boosted by the winners of the giveaway
    pub boosted_chat_id: i64,
    /// Identifier of the message with the giveaway in the boosted chat; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// True, if the corresponding winner wasn't chosen and the Telegram Stars were received by the owner of the boosted chat
    pub is_unclaimed: bool,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// A message with an invoice; can be used only by bots
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageInvoice {
    /// Invoice
    pub invoice: crate::types::Invoice,
    /// Product title; 1-32 characters
    pub title: String,
    /// Product description; 0-255 characters
    pub description: String,
    /// Product photo URL; optional
    pub photo_url: String,
    /// Product photo size
    pub photo_size: i32,
    /// Product photo width
    pub photo_width: i32,
    /// Product photo height
    pub photo_height: i32,
    /// The invoice payload
    pub payload: String,
    /// Payment provider token; may be empty for payments in Telegram Stars
    pub provider_token: String,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider
    pub provider_data: String,
    /// Unique invoice bot deep link parameter for the generation of this invoice. If empty, it would be possible to pay directly from forwards of the invoice message
    pub start_parameter: String,
    /// The content of paid media attached to the invoice; pass null if none
    pub paid_media: Option<crate::types::InputPaidMedia>,
    /// Paid media caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub paid_media_caption: Option<crate::types::FormattedText>,
}

/// The user subscribing to Telegram Premium
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumSubscription {
    /// Pass true if this is a restore of a Telegram Premium purchase; only for App Store
    pub is_restore: bool,
    /// Pass true if this is an upgrade from a monthly subscription to early subscription; only for App Store
    pub is_upgrade: bool,
}

/// The user gifting Telegram Premium to another user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGift {
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifier of the user who will receive Telegram Premium
    pub user_id: i64,
    /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
    pub text: crate::types::FormattedText,
}

/// The user boosting a chat by creating Telegram Premium gift codes for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGiftCodes {
    /// Identifier of the supergroup or channel chat, which will be automatically boosted by the users for duration of the Premium subscription and which is administered by the user
    pub boosted_chat_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifiers of the users who can activate the gift codes
    pub user_ids: Vec<i64>,
    /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
    pub text: crate::types::FormattedText,
}

/// The user creating a Telegram Premium giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
}

/// The user creating a Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposeStarGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// The number of users to receive Telegram Stars
    pub winner_count: i32,
    /// The number of Telegram Stars to be distributed through the giveaway
    pub star_count: i64,
}

/// The user buying Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposeStars {
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
    /// Identifier of the chat that is supposed to receive the Telegram Stars; pass 0 if none
    pub chat_id: i64,
}

/// The user buying Telegram Stars for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposeGiftedStars {
    /// Identifier of the user to which Telegram Stars are gifted
    pub user_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
}

/// The user gifting Telegram Premium to another user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposePremiumGift {
    /// ISO 4217 currency code of the payment currency, or "XTR" for payments in Telegram Stars
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifier of the user who will receive Telegram Premium
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active for the user
    pub month_count: i32,
    /// Text to show to the user receiving Telegram Premium; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
    pub text: crate::types::FormattedText,
}

/// The user boosting a chat by creating Telegram Premium gift codes for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposePremiumGiftCodes {
    /// Identifier of the supergroup or channel chat, which will be automatically boosted by the users for duration of the Premium subscription and which is administered by the user
    pub boosted_chat_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifiers of the users who can activate the gift codes
    pub user_ids: Vec<i64>,
    /// Number of months the Telegram Premium subscription will be active for the users
    pub month_count: i32,
    /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
    pub text: crate::types::FormattedText,
}

/// The user creating a Telegram Premium giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposePremiumGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of users who will be able to activate the gift codes
    pub winner_count: i32,
    /// Number of months the Telegram Premium subscription will be active for the users
    pub month_count: i32,
}

/// The user buying Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeStars {
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
    /// Identifier of the chat that is supposed to receive the Telegram Stars; pass 0 if none
    pub chat_id: i64,
}

/// The user buying Telegram Stars for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeGiftedStars {
    /// Identifier of the user to which Telegram Stars are gifted
    pub user_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Number of bought Telegram Stars
    pub star_count: i64,
}

/// The user creating a Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeStarGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// The number of users to receive Telegram Stars
    pub winner_count: i32,
    /// The number of Telegram Stars to be distributed through the giveaway
    pub star_count: i64,
}

/// The user joins a chat and subscribes to regular payments in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposeJoinChat {
    /// Invite link to use
    pub invite_link: String,
}

/// A message with an invoice from a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentInvoice {
    /// Product price
    pub price: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A new recurring payment was made by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentRecurringPayment {
    /// The paid amount
    pub amount: String,
}

/// The Telegram Star balance and transaction section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionMyStars {
    /// Subsection of the section; may be one of
    /// "", "top-up", "stats", "gift", "earn"
    pub subsection: String,
}

/// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeInvoice {
    /// Name of the invoice
    pub invoice_name: String,
}

/// Suggests the user to extend their expiring Telegram Star subscriptions. Call getStarSubscriptions with only_expiring == true
/// to get the number of expiring subscriptions and the number of required to buy Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionExtendStarSubscriptions {
}

/// Contains information about revenue earned from sponsored messages in a chat
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueAmount {
    /// Cryptocurrency in which revenue is calculated
    pub cryptocurrency: String,
    /// Total amount of the cryptocurrency earned, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: i64,
    /// Amount of the cryptocurrency that isn't withdrawn yet, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub balance_amount: i64,
    /// Amount of the cryptocurrency available for withdrawal, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub available_amount: i64,
    /// True, if Telegram Stars can be withdrawn now or later
    pub withdrawal_enabled: bool,
}

/// A detailed statistics about revenue earned from sponsored messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueStatistics {
    /// A graph containing amount of revenue in a given hour
    pub revenue_by_hour_graph: crate::enums::StatisticalGraph,
    /// A graph containing amount of revenue
    pub revenue_graph: crate::enums::StatisticalGraph,
    /// Amount of earned revenue
    pub revenue_amount: crate::types::ChatRevenueAmount,
    /// Current conversion rate of the cryptocurrency in which revenue is calculated to USD
    pub usd_rate: f64,
}

/// Withdrawal is pending
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStatePending {
}

/// Withdrawal succeeded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Point in time (Unix timestamp) when the withdrawal was completed
    pub date: i32,
    /// The URL where the withdrawal transaction can be viewed
    pub url: String,
}

/// Withdrawal failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStateFailed {
}

/// Describes an unsupported transaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeUnsupported {
}

/// Describes earnings from sponsored messages in a chat in some time frame
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeSponsoredMessageEarnings {
    /// Point in time (Unix timestamp) when the earnings started
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the earnings ended
    pub end_date: i32,
}

/// Describes earnings from a published suggested post
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeSuggestedPostEarnings {
    /// Identifier of the user who paid for the suggested post
    pub user_id: i64,
}

/// Describes a withdrawal of earnings through Fragment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeFragmentWithdrawal {
    /// Point in time (Unix timestamp) when the earnings withdrawal started
    pub withdrawal_date: i32,
    /// State of the withdrawal
    pub state: crate::enums::RevenueWithdrawalState,
}

/// Describes a refund for failed withdrawal of earnings through Fragment
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeFragmentRefund {
    /// Point in time (Unix timestamp) when the transaction was refunded
    pub refund_date: i32,
}

/// Contains a chat revenue transactions
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransaction {
    /// Cryptocurrency in which revenue is calculated
    pub cryptocurrency: String,
    /// The withdrawn amount, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Type of the transaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::ChatRevenueTransactionType,
}

/// Contains a list of chat revenue transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactions {
    /// The amount of owned TON Grams; in the smallest units of the cryptocurrency
    pub gram_amount: i64,
    /// List of transactions
    pub transactions: Vec<crate::types::ChatRevenueTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains information about Telegram Stars earned by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarRevenueStatus {
    /// Total Telegram Star amount earned
    pub total_amount: crate::types::StarAmount,
    /// The Telegram Star amount that isn't withdrawn yet
    pub current_amount: crate::types::StarAmount,
    /// The Telegram Star amount that is available for withdrawal
    pub available_amount: crate::types::StarAmount,
    /// True, if Telegram Stars can be withdrawn now or later
    pub withdrawal_enabled: bool,
    /// Time left before the next withdrawal can be started, in seconds; 0 if withdrawal can be started now
    pub next_withdrawal_in: i32,
}

/// A detailed statistics about Telegram Stars earned by a user or a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarRevenueStatistics {
    /// A graph containing amount of revenue in a given day
    pub revenue_by_day_graph: crate::enums::StatisticalGraph,
    /// Telegram Star revenue status
    pub status: crate::types::StarRevenueStatus,
    /// Current conversion rate of a Telegram Star to USD
    pub usd_rate: f64,
}

/// Contains information about TON Grams earned by the current user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GramRevenueStatus {
    /// Total Gram amount earned; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: i64,
    /// The Gram amount that isn't withdrawn yet; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub balance_amount: i64,
    /// The Gram amount that is available for withdrawal; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub available_amount: i64,
    /// True, if Grams can be withdrawn
    pub withdrawal_enabled: bool,
}

/// A detailed statistics about TON Grams earned by the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GramRevenueStatistics {
    /// A graph containing amount of revenue in a given day
    pub revenue_by_day_graph: crate::enums::StatisticalGraph,
    /// Amount of earned revenue
    pub status: crate::types::GramRevenueStatus,
    /// Current conversion rate of nanogram to USD cents
    pub usd_rate: f64,
}

/// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatRevenueAmount {
    /// Identifier of the chat
    pub chat_id: i64,
    /// New amount of earned revenue
    pub revenue_amount: crate::types::ChatRevenueAmount,
}

/// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStarRevenueStatus {
    /// Identifier of the owner of the Telegram Stars
    pub owner_id: crate::enums::MessageSender,
    /// New Telegram Star revenue status
    pub status: crate::types::StarRevenueStatus,
}

/// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGramRevenueStatus {
    /// New Gram revenue status
    pub status: crate::types::GramRevenueStatus,
}

/// A new incoming shipping query; for bots only. Only for invoices with flexible price
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewShippingQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Invoice payload
    pub invoice_payload: String,
    /// User shipping address
    pub shipping_address: crate::types::Address,
}

/// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewPreCheckoutQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Currency for the product price
    pub currency: String,
    /// Total price for the product, in the smallest units of the currency
    pub total_amount: i64,
    /// Invoice payload
    pub invoice_payload: String,
    /// Identifier of a shipping option chosen by the user; may be empty if not applicable
    pub shipping_option_id: String,
    /// Information about the order; may be null
    pub order_info: Option<crate::types::OrderInfo>,
}

