//!
//! TDLib `payment` domain enums.
//!
//! Types, enums, and functions for invoices, payments, Telegram Stars, and monetized features.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes type of subscription paid in Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptionType {
    /// Describes a subscription to a channel chat
    #[serde(rename(serialize = "starSubscriptionTypeChannel", deserialize = "starSubscriptionTypeChannel"))]
    Channel(Box<crate::types::StarSubscriptionTypeChannel>),
    /// Describes a subscription in a bot or a business account
    #[serde(rename(serialize = "starSubscriptionTypeBot", deserialize = "starSubscriptionTypeBot"))]
    Bot(Box<crate::types::StarSubscriptionTypeBot>),
}

impl StarSubscriptionType {
    /// Convenience constructor to create a [`StarSubscriptionType::Channel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel(val: crate::types::StarSubscriptionTypeChannel) -> Self {
        Self::Channel(Box::new(val))
    }

    /// Convenience constructor to create a [`StarSubscriptionType::Bot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot(val: crate::types::StarSubscriptionTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }

}

/// Converts a [`crate::types::StarSubscriptionTypeChannel`] into [`StarSubscriptionType`].
impl From<crate::types::StarSubscriptionTypeChannel> for StarSubscriptionType {
    fn from(val: crate::types::StarSubscriptionTypeChannel) -> Self {
        Self::Channel(Box::new(val))
    }
}

/// Converts a [`crate::types::StarSubscriptionTypeBot`] into [`StarSubscriptionType`].
impl From<crate::types::StarSubscriptionTypeBot> for StarSubscriptionType {
    fn from(val: crate::types::StarSubscriptionTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }
}

/// TDLib `StarSubscriptionPricing` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptionPricing {
    /// Describes subscription plan paid in Telegram Stars
    #[serde(rename(serialize = "starSubscriptionPricing", deserialize = "starSubscriptionPricing"))]
    StarSubscriptionPricing(Box<crate::types::StarSubscriptionPricing>),
}

impl StarSubscriptionPricing {
    /// Convenience constructor to create a [`StarSubscriptionPricing::StarSubscriptionPricing`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_subscription_pricing(val: crate::types::StarSubscriptionPricing) -> Self {
        Self::StarSubscriptionPricing(Box::new(val))
    }

}

/// Converts a [`crate::types::StarSubscriptionPricing`] into [`StarSubscriptionPricing`].
impl From<crate::types::StarSubscriptionPricing> for StarSubscriptionPricing {
    fn from(val: crate::types::StarSubscriptionPricing) -> Self {
        Self::StarSubscriptionPricing(Box::new(val))
    }
}

/// TDLib `StarSubscription` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscription {
    /// Contains information about subscription to a channel chat, a bot, or a business account that was paid in Telegram Stars
    #[serde(rename(serialize = "starSubscription", deserialize = "starSubscription"))]
    StarSubscription(Box<crate::types::StarSubscription>),
}

impl StarSubscription {
    /// Convenience constructor to create a [`StarSubscription::StarSubscription`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_subscription(val: crate::types::StarSubscription) -> Self {
        Self::StarSubscription(Box::new(val))
    }

}

/// Converts a [`crate::types::StarSubscription`] into [`StarSubscription`].
impl From<crate::types::StarSubscription> for StarSubscription {
    fn from(val: crate::types::StarSubscription) -> Self {
        Self::StarSubscription(Box::new(val))
    }
}

/// TDLib `StarSubscriptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptions {
    /// Represents a list of Telegram Star subscriptions
    #[serde(rename(serialize = "starSubscriptions", deserialize = "starSubscriptions"))]
    StarSubscriptions(Box<crate::types::StarSubscriptions>),
}

impl StarSubscriptions {
    /// Convenience constructor to create a [`StarSubscriptions::StarSubscriptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_subscriptions(val: crate::types::StarSubscriptions) -> Self {
        Self::StarSubscriptions(Box::new(val))
    }

}

/// Converts a [`crate::types::StarSubscriptions`] into [`StarSubscriptions`].
impl From<crate::types::StarSubscriptions> for StarSubscriptions {
    fn from(val: crate::types::StarSubscriptions) -> Self {
        Self::StarSubscriptions(Box::new(val))
    }
}

/// TDLib `PremiumPaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumPaymentOption {
    /// Describes an option for buying Telegram Premium to a user
    #[serde(rename(serialize = "premiumPaymentOption", deserialize = "premiumPaymentOption"))]
    PremiumPaymentOption(Box<crate::types::PremiumPaymentOption>),
}

impl PremiumPaymentOption {
    /// Convenience constructor to create a [`PremiumPaymentOption::PremiumPaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_payment_option(val: crate::types::PremiumPaymentOption) -> Self {
        Self::PremiumPaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumPaymentOption`] into [`PremiumPaymentOption`].
impl From<crate::types::PremiumPaymentOption> for PremiumPaymentOption {
    fn from(val: crate::types::PremiumPaymentOption) -> Self {
        Self::PremiumPaymentOption(Box::new(val))
    }
}

/// TDLib `PremiumStatePaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumStatePaymentOption {
    /// Describes an option for buying or upgrading Telegram Premium for self
    #[serde(rename(serialize = "premiumStatePaymentOption", deserialize = "premiumStatePaymentOption"))]
    PremiumStatePaymentOption(Box<crate::types::PremiumStatePaymentOption>),
}

impl PremiumStatePaymentOption {
    /// Convenience constructor to create a [`PremiumStatePaymentOption::PremiumStatePaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_state_payment_option(val: crate::types::PremiumStatePaymentOption) -> Self {
        Self::PremiumStatePaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumStatePaymentOption`] into [`PremiumStatePaymentOption`].
impl From<crate::types::PremiumStatePaymentOption> for PremiumStatePaymentOption {
    fn from(val: crate::types::PremiumStatePaymentOption) -> Self {
        Self::PremiumStatePaymentOption(Box::new(val))
    }
}

/// TDLib `PremiumGiftPaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftPaymentOption {
    /// Describes an option for gifting Telegram Premium to a user. Use telegramPaymentPurposePremiumGift for out-of-store payments or payments in Telegram Stars
    #[serde(rename(serialize = "premiumGiftPaymentOption", deserialize = "premiumGiftPaymentOption"))]
    PremiumGiftPaymentOption(Box<crate::types::PremiumGiftPaymentOption>),
}

impl PremiumGiftPaymentOption {
    /// Convenience constructor to create a [`PremiumGiftPaymentOption::PremiumGiftPaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_payment_option(val: crate::types::PremiumGiftPaymentOption) -> Self {
        Self::PremiumGiftPaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumGiftPaymentOption`] into [`PremiumGiftPaymentOption`].
impl From<crate::types::PremiumGiftPaymentOption> for PremiumGiftPaymentOption {
    fn from(val: crate::types::PremiumGiftPaymentOption) -> Self {
        Self::PremiumGiftPaymentOption(Box::new(val))
    }
}

/// TDLib `PremiumGiftPaymentOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftPaymentOptions {
    /// Contains a list of options for gifting Telegram Premium to a user
    #[serde(rename(serialize = "premiumGiftPaymentOptions", deserialize = "premiumGiftPaymentOptions"))]
    PremiumGiftPaymentOptions(Box<crate::types::PremiumGiftPaymentOptions>),
}

impl PremiumGiftPaymentOptions {
    /// Convenience constructor to create a [`PremiumGiftPaymentOptions::PremiumGiftPaymentOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_payment_options(val: crate::types::PremiumGiftPaymentOptions) -> Self {
        Self::PremiumGiftPaymentOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumGiftPaymentOptions`] into [`PremiumGiftPaymentOptions`].
impl From<crate::types::PremiumGiftPaymentOptions> for PremiumGiftPaymentOptions {
    fn from(val: crate::types::PremiumGiftPaymentOptions) -> Self {
        Self::PremiumGiftPaymentOptions(Box::new(val))
    }
}

/// TDLib `PremiumGiveawayPaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayPaymentOption {
    /// Describes an option for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members. Use telegramPaymentPurposePremiumGiftCodes or telegramPaymentPurposePremiumGiveaway for out-of-store payments
    #[serde(rename(serialize = "premiumGiveawayPaymentOption", deserialize = "premiumGiveawayPaymentOption"))]
    PremiumGiveawayPaymentOption(Box<crate::types::PremiumGiveawayPaymentOption>),
}

impl PremiumGiveawayPaymentOption {
    /// Convenience constructor to create a [`PremiumGiveawayPaymentOption::PremiumGiveawayPaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_giveaway_payment_option(val: crate::types::PremiumGiveawayPaymentOption) -> Self {
        Self::PremiumGiveawayPaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumGiveawayPaymentOption`] into [`PremiumGiveawayPaymentOption`].
impl From<crate::types::PremiumGiveawayPaymentOption> for PremiumGiveawayPaymentOption {
    fn from(val: crate::types::PremiumGiveawayPaymentOption) -> Self {
        Self::PremiumGiveawayPaymentOption(Box::new(val))
    }
}

/// TDLib `PremiumGiveawayPaymentOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayPaymentOptions {
    /// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
    #[serde(rename(serialize = "premiumGiveawayPaymentOptions", deserialize = "premiumGiveawayPaymentOptions"))]
    PremiumGiveawayPaymentOptions(Box<crate::types::PremiumGiveawayPaymentOptions>),
}

impl PremiumGiveawayPaymentOptions {
    /// Convenience constructor to create a [`PremiumGiveawayPaymentOptions::PremiumGiveawayPaymentOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_giveaway_payment_options(val: crate::types::PremiumGiveawayPaymentOptions) -> Self {
        Self::PremiumGiveawayPaymentOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumGiveawayPaymentOptions`] into [`PremiumGiveawayPaymentOptions`].
impl From<crate::types::PremiumGiveawayPaymentOptions> for PremiumGiveawayPaymentOptions {
    fn from(val: crate::types::PremiumGiveawayPaymentOptions) -> Self {
        Self::PremiumGiveawayPaymentOptions(Box::new(val))
    }
}

/// TDLib `StarPaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarPaymentOption {
    /// Describes an option for buying Telegram Stars. Use telegramPaymentPurposeStars for out-of-store payments
    #[serde(rename(serialize = "starPaymentOption", deserialize = "starPaymentOption"))]
    StarPaymentOption(Box<crate::types::StarPaymentOption>),
}

impl StarPaymentOption {
    /// Convenience constructor to create a [`StarPaymentOption::StarPaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_payment_option(val: crate::types::StarPaymentOption) -> Self {
        Self::StarPaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::StarPaymentOption`] into [`StarPaymentOption`].
impl From<crate::types::StarPaymentOption> for StarPaymentOption {
    fn from(val: crate::types::StarPaymentOption) -> Self {
        Self::StarPaymentOption(Box::new(val))
    }
}

/// TDLib `StarPaymentOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarPaymentOptions {
    /// Contains a list of options for buying Telegram Stars
    #[serde(rename(serialize = "starPaymentOptions", deserialize = "starPaymentOptions"))]
    StarPaymentOptions(Box<crate::types::StarPaymentOptions>),
}

impl StarPaymentOptions {
    /// Convenience constructor to create a [`StarPaymentOptions::StarPaymentOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_payment_options(val: crate::types::StarPaymentOptions) -> Self {
        Self::StarPaymentOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::StarPaymentOptions`] into [`StarPaymentOptions`].
impl From<crate::types::StarPaymentOptions> for StarPaymentOptions {
    fn from(val: crate::types::StarPaymentOptions) -> Self {
        Self::StarPaymentOptions(Box::new(val))
    }
}

/// TDLib `StarGiveawayPaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayPaymentOption {
    /// Describes an option for creating of Telegram Star giveaway. Use telegramPaymentPurposeStarGiveaway for out-of-store payments
    #[serde(rename(serialize = "starGiveawayPaymentOption", deserialize = "starGiveawayPaymentOption"))]
    StarGiveawayPaymentOption(Box<crate::types::StarGiveawayPaymentOption>),
}

impl StarGiveawayPaymentOption {
    /// Convenience constructor to create a [`StarGiveawayPaymentOption::StarGiveawayPaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_giveaway_payment_option(val: crate::types::StarGiveawayPaymentOption) -> Self {
        Self::StarGiveawayPaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::StarGiveawayPaymentOption`] into [`StarGiveawayPaymentOption`].
impl From<crate::types::StarGiveawayPaymentOption> for StarGiveawayPaymentOption {
    fn from(val: crate::types::StarGiveawayPaymentOption) -> Self {
        Self::StarGiveawayPaymentOption(Box::new(val))
    }
}

/// TDLib `StarGiveawayPaymentOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayPaymentOptions {
    /// Contains a list of options for creating of Telegram Star giveaway
    #[serde(rename(serialize = "starGiveawayPaymentOptions", deserialize = "starGiveawayPaymentOptions"))]
    StarGiveawayPaymentOptions(Box<crate::types::StarGiveawayPaymentOptions>),
}

impl StarGiveawayPaymentOptions {
    /// Convenience constructor to create a [`StarGiveawayPaymentOptions::StarGiveawayPaymentOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_giveaway_payment_options(val: crate::types::StarGiveawayPaymentOptions) -> Self {
        Self::StarGiveawayPaymentOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::StarGiveawayPaymentOptions`] into [`StarGiveawayPaymentOptions`].
impl From<crate::types::StarGiveawayPaymentOptions> for StarGiveawayPaymentOptions {
    fn from(val: crate::types::StarGiveawayPaymentOptions) -> Self {
        Self::StarGiveawayPaymentOptions(Box::new(val))
    }
}

/// TDLib `Invoice` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Invoice {
    /// Product invoice
    #[serde(rename(serialize = "invoice", deserialize = "invoice"))]
    Invoice(Box<crate::types::Invoice>),
}

impl Invoice {
    /// Convenience constructor to create a [`Invoice::Invoice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn invoice(val: crate::types::Invoice) -> Self {
        Self::Invoice(Box::new(val))
    }

}

/// Converts a [`crate::types::Invoice`] into [`Invoice`].
impl From<crate::types::Invoice> for Invoice {
    fn from(val: crate::types::Invoice) -> Self {
        Self::Invoice(Box::new(val))
    }
}

/// TDLib `ShippingOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ShippingOption {
    /// One shipping option
    #[serde(rename(serialize = "shippingOption", deserialize = "shippingOption"))]
    ShippingOption(Box<crate::types::ShippingOption>),
}

impl ShippingOption {
    /// Convenience constructor to create a [`ShippingOption::ShippingOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn shipping_option(val: crate::types::ShippingOption) -> Self {
        Self::ShippingOption(Box::new(val))
    }

}

/// Converts a [`crate::types::ShippingOption`] into [`ShippingOption`].
impl From<crate::types::ShippingOption> for ShippingOption {
    fn from(val: crate::types::ShippingOption) -> Self {
        Self::ShippingOption(Box::new(val))
    }
}

/// Contains information about a payment provider
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentProvider {
    /// Smart Glocal payment provider
    #[serde(rename(serialize = "paymentProviderSmartGlocal", deserialize = "paymentProviderSmartGlocal"))]
    SmartGlocal(Box<crate::types::PaymentProviderSmartGlocal>),
    /// Stripe payment provider
    #[serde(rename(serialize = "paymentProviderStripe", deserialize = "paymentProviderStripe"))]
    Stripe(Box<crate::types::PaymentProviderStripe>),
    /// Some other payment provider, for which a web payment form must be shown
    #[serde(rename(serialize = "paymentProviderOther", deserialize = "paymentProviderOther"))]
    Other(Box<crate::types::PaymentProviderOther>),
}

impl PaymentProvider {
    /// Convenience constructor to create a [`PaymentProvider::SmartGlocal`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn smart_glocal(val: crate::types::PaymentProviderSmartGlocal) -> Self {
        Self::SmartGlocal(Box::new(val))
    }

    /// Convenience constructor to create a [`PaymentProvider::Stripe`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stripe(val: crate::types::PaymentProviderStripe) -> Self {
        Self::Stripe(Box::new(val))
    }

    /// Convenience constructor to create a [`PaymentProvider::Other`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn other(val: crate::types::PaymentProviderOther) -> Self {
        Self::Other(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentProviderSmartGlocal`] into [`PaymentProvider`].
impl From<crate::types::PaymentProviderSmartGlocal> for PaymentProvider {
    fn from(val: crate::types::PaymentProviderSmartGlocal) -> Self {
        Self::SmartGlocal(Box::new(val))
    }
}

/// Converts a [`crate::types::PaymentProviderStripe`] into [`PaymentProvider`].
impl From<crate::types::PaymentProviderStripe> for PaymentProvider {
    fn from(val: crate::types::PaymentProviderStripe) -> Self {
        Self::Stripe(Box::new(val))
    }
}

/// Converts a [`crate::types::PaymentProviderOther`] into [`PaymentProvider`].
impl From<crate::types::PaymentProviderOther> for PaymentProvider {
    fn from(val: crate::types::PaymentProviderOther) -> Self {
        Self::Other(Box::new(val))
    }
}

/// TDLib `PaymentOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentOption {
    /// Describes an additional payment option
    #[serde(rename(serialize = "paymentOption", deserialize = "paymentOption"))]
    PaymentOption(Box<crate::types::PaymentOption>),
}

impl PaymentOption {
    /// Convenience constructor to create a [`PaymentOption::PaymentOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn payment_option(val: crate::types::PaymentOption) -> Self {
        Self::PaymentOption(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentOption`] into [`PaymentOption`].
impl From<crate::types::PaymentOption> for PaymentOption {
    fn from(val: crate::types::PaymentOption) -> Self {
        Self::PaymentOption(Box::new(val))
    }
}

/// Describes type of payment form
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentFormType {
    /// The payment form is for a regular payment
    #[serde(rename(serialize = "paymentFormTypeRegular", deserialize = "paymentFormTypeRegular"))]
    Regular(Box<crate::types::PaymentFormTypeRegular>),
    /// The payment form is for a payment in Telegram Stars
    #[serde(rename(serialize = "paymentFormTypeStars", deserialize = "paymentFormTypeStars"))]
    Stars(Box<crate::types::PaymentFormTypeStars>),
    /// The payment form is for a payment in Telegram Stars for subscription
    #[serde(rename(serialize = "paymentFormTypeStarSubscription", deserialize = "paymentFormTypeStarSubscription"))]
    StarSubscription(Box<crate::types::PaymentFormTypeStarSubscription>),
}

impl PaymentFormType {
    /// Convenience constructor to create a [`PaymentFormType::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::PaymentFormTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`PaymentFormType::Stars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stars(val: crate::types::PaymentFormTypeStars) -> Self {
        Self::Stars(Box::new(val))
    }

    /// Convenience constructor to create a [`PaymentFormType::StarSubscription`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_subscription(val: crate::types::PaymentFormTypeStarSubscription) -> Self {
        Self::StarSubscription(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentFormTypeRegular`] into [`PaymentFormType`].
impl From<crate::types::PaymentFormTypeRegular> for PaymentFormType {
    fn from(val: crate::types::PaymentFormTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::PaymentFormTypeStars`] into [`PaymentFormType`].
impl From<crate::types::PaymentFormTypeStars> for PaymentFormType {
    fn from(val: crate::types::PaymentFormTypeStars) -> Self {
        Self::Stars(Box::new(val))
    }
}

/// Converts a [`crate::types::PaymentFormTypeStarSubscription`] into [`PaymentFormType`].
impl From<crate::types::PaymentFormTypeStarSubscription> for PaymentFormType {
    fn from(val: crate::types::PaymentFormTypeStarSubscription) -> Self {
        Self::StarSubscription(Box::new(val))
    }
}

/// TDLib `PaymentForm` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentForm {
    /// Contains information about an invoice payment form
    #[serde(rename(serialize = "paymentForm", deserialize = "paymentForm"))]
    PaymentForm(Box<crate::types::PaymentForm>),
}

impl PaymentForm {
    /// Convenience constructor to create a [`PaymentForm::PaymentForm`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn payment_form(val: crate::types::PaymentForm) -> Self {
        Self::PaymentForm(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentForm`] into [`PaymentForm`].
impl From<crate::types::PaymentForm> for PaymentForm {
    fn from(val: crate::types::PaymentForm) -> Self {
        Self::PaymentForm(Box::new(val))
    }
}

/// TDLib `PaymentResult` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentResult {
    /// Contains the result of a payment request
    #[serde(rename(serialize = "paymentResult", deserialize = "paymentResult"))]
    PaymentResult(Box<crate::types::PaymentResult>),
}

impl PaymentResult {
    /// Convenience constructor to create a [`PaymentResult::PaymentResult`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn payment_result(val: crate::types::PaymentResult) -> Self {
        Self::PaymentResult(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentResult`] into [`PaymentResult`].
impl From<crate::types::PaymentResult> for PaymentResult {
    fn from(val: crate::types::PaymentResult) -> Self {
        Self::PaymentResult(Box::new(val))
    }
}

/// Describes type of successful payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentReceiptType {
    /// The payment was done using a third-party payment provider
    #[serde(rename(serialize = "paymentReceiptTypeRegular", deserialize = "paymentReceiptTypeRegular"))]
    Regular(Box<crate::types::PaymentReceiptTypeRegular>),
    /// The payment was done using Telegram Stars
    #[serde(rename(serialize = "paymentReceiptTypeStars", deserialize = "paymentReceiptTypeStars"))]
    Stars(Box<crate::types::PaymentReceiptTypeStars>),
}

impl PaymentReceiptType {
    /// Convenience constructor to create a [`PaymentReceiptType::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::PaymentReceiptTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`PaymentReceiptType::Stars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stars(val: crate::types::PaymentReceiptTypeStars) -> Self {
        Self::Stars(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentReceiptTypeRegular`] into [`PaymentReceiptType`].
impl From<crate::types::PaymentReceiptTypeRegular> for PaymentReceiptType {
    fn from(val: crate::types::PaymentReceiptTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::PaymentReceiptTypeStars`] into [`PaymentReceiptType`].
impl From<crate::types::PaymentReceiptTypeStars> for PaymentReceiptType {
    fn from(val: crate::types::PaymentReceiptTypeStars) -> Self {
        Self::Stars(Box::new(val))
    }
}

/// TDLib `PaymentReceipt` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentReceipt {
    /// Contains information about a successful payment
    #[serde(rename(serialize = "paymentReceipt", deserialize = "paymentReceipt"))]
    PaymentReceipt(Box<crate::types::PaymentReceipt>),
}

impl PaymentReceipt {
    /// Convenience constructor to create a [`PaymentReceipt::PaymentReceipt`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn payment_receipt(val: crate::types::PaymentReceipt) -> Self {
        Self::PaymentReceipt(Box::new(val))
    }

}

/// Converts a [`crate::types::PaymentReceipt`] into [`PaymentReceipt`].
impl From<crate::types::PaymentReceipt> for PaymentReceipt {
    fn from(val: crate::types::PaymentReceipt) -> Self {
        Self::PaymentReceipt(Box::new(val))
    }
}

/// Describes an invoice to process
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputInvoice {
    /// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
    #[serde(rename(serialize = "inputInvoiceMessage", deserialize = "inputInvoiceMessage"))]
    Message(Box<crate::types::InputInvoiceMessage>),
    /// An invoice from a link of the type internalLinkTypeInvoice
    #[serde(rename(serialize = "inputInvoiceName", deserialize = "inputInvoiceName"))]
    Name(Box<crate::types::InputInvoiceName>),
    /// An invoice for a payment toward Telegram; must not be used in the in-store apps
    #[serde(rename(serialize = "inputInvoiceTelegram", deserialize = "inputInvoiceTelegram"))]
    Telegram(Box<crate::types::InputInvoiceTelegram>),
}

impl InputInvoice {
    /// Convenience constructor to create a [`InputInvoice::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::InputInvoiceMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInvoice::Name`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn name(val: crate::types::InputInvoiceName) -> Self {
        Self::Name(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInvoice::Telegram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn telegram(val: crate::types::InputInvoiceTelegram) -> Self {
        Self::Telegram(Box::new(val))
    }

}

/// Converts a [`crate::types::InputInvoiceMessage`] into [`InputInvoice`].
impl From<crate::types::InputInvoiceMessage> for InputInvoice {
    fn from(val: crate::types::InputInvoiceMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInvoiceName`] into [`InputInvoice`].
impl From<crate::types::InputInvoiceName> for InputInvoice {
    fn from(val: crate::types::InputInvoiceName) -> Self {
        Self::Name(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInvoiceTelegram`] into [`InputInvoice`].
impl From<crate::types::InputInvoiceTelegram> for InputInvoice {
    fn from(val: crate::types::InputInvoiceTelegram) -> Self {
        Self::Telegram(Box::new(val))
    }
}

/// Describes a purpose of an in-store payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorePaymentPurpose {
    /// The user subscribing to Telegram Premium
    #[serde(rename(serialize = "storePaymentPurposePremiumSubscription", deserialize = "storePaymentPurposePremiumSubscription"))]
    PremiumSubscription(Box<crate::types::StorePaymentPurposePremiumSubscription>),
    /// The user gifting Telegram Premium to another user
    #[serde(rename(serialize = "storePaymentPurposePremiumGift", deserialize = "storePaymentPurposePremiumGift"))]
    PremiumGift(Box<crate::types::StorePaymentPurposePremiumGift>),
    /// The user boosting a chat by creating Telegram Premium gift codes for other users
    #[serde(rename(serialize = "storePaymentPurposePremiumGiftCodes", deserialize = "storePaymentPurposePremiumGiftCodes"))]
    PremiumGiftCodes(Box<crate::types::StorePaymentPurposePremiumGiftCodes>),
    /// The user creating a Telegram Premium giveaway
    #[serde(rename(serialize = "storePaymentPurposePremiumGiveaway", deserialize = "storePaymentPurposePremiumGiveaway"))]
    PremiumGiveaway(Box<crate::types::StorePaymentPurposePremiumGiveaway>),
    /// The user creating a Telegram Star giveaway
    #[serde(rename(serialize = "storePaymentPurposeStarGiveaway", deserialize = "storePaymentPurposeStarGiveaway"))]
    StarGiveaway(Box<crate::types::StorePaymentPurposeStarGiveaway>),
    /// The user buying Telegram Stars
    #[serde(rename(serialize = "storePaymentPurposeStars", deserialize = "storePaymentPurposeStars"))]
    Stars(Box<crate::types::StorePaymentPurposeStars>),
    /// The user buying Telegram Stars for other users
    #[serde(rename(serialize = "storePaymentPurposeGiftedStars", deserialize = "storePaymentPurposeGiftedStars"))]
    GiftedStars(Box<crate::types::StorePaymentPurposeGiftedStars>),
}

impl StorePaymentPurpose {
    /// Convenience constructor to create a [`StorePaymentPurpose::PremiumSubscription`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_subscription(val: crate::types::StorePaymentPurposePremiumSubscription) -> Self {
        Self::PremiumSubscription(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::PremiumGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift(val: crate::types::StorePaymentPurposePremiumGift) -> Self {
        Self::PremiumGift(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::PremiumGiftCodes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_codes(val: crate::types::StorePaymentPurposePremiumGiftCodes) -> Self {
        Self::PremiumGiftCodes(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::PremiumGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_giveaway(val: crate::types::StorePaymentPurposePremiumGiveaway) -> Self {
        Self::PremiumGiveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::StarGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_giveaway(val: crate::types::StorePaymentPurposeStarGiveaway) -> Self {
        Self::StarGiveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::Stars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stars(val: crate::types::StorePaymentPurposeStars) -> Self {
        Self::Stars(Box::new(val))
    }

    /// Convenience constructor to create a [`StorePaymentPurpose::GiftedStars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gifted_stars(val: crate::types::StorePaymentPurposeGiftedStars) -> Self {
        Self::GiftedStars(Box::new(val))
    }

}

/// Converts a [`crate::types::StorePaymentPurposePremiumSubscription`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposePremiumSubscription> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposePremiumSubscription) -> Self {
        Self::PremiumSubscription(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposePremiumGift`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposePremiumGift> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposePremiumGift) -> Self {
        Self::PremiumGift(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposePremiumGiftCodes`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposePremiumGiftCodes> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposePremiumGiftCodes) -> Self {
        Self::PremiumGiftCodes(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposePremiumGiveaway`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposePremiumGiveaway> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposePremiumGiveaway) -> Self {
        Self::PremiumGiveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposeStarGiveaway`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposeStarGiveaway> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposeStarGiveaway) -> Self {
        Self::StarGiveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposeStars`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposeStars> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposeStars) -> Self {
        Self::Stars(Box::new(val))
    }
}

/// Converts a [`crate::types::StorePaymentPurposeGiftedStars`] into [`StorePaymentPurpose`].
impl From<crate::types::StorePaymentPurposeGiftedStars> for StorePaymentPurpose {
    fn from(val: crate::types::StorePaymentPurposeGiftedStars) -> Self {
        Self::GiftedStars(Box::new(val))
    }
}

/// Describes a purpose of a payment toward Telegram
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TelegramPaymentPurpose {
    /// The user gifting Telegram Premium to another user
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGift", deserialize = "telegramPaymentPurposePremiumGift"))]
    PremiumGift(Box<crate::types::TelegramPaymentPurposePremiumGift>),
    /// The user boosting a chat by creating Telegram Premium gift codes for other users
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGiftCodes", deserialize = "telegramPaymentPurposePremiumGiftCodes"))]
    PremiumGiftCodes(Box<crate::types::TelegramPaymentPurposePremiumGiftCodes>),
    /// The user creating a Telegram Premium giveaway
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGiveaway", deserialize = "telegramPaymentPurposePremiumGiveaway"))]
    PremiumGiveaway(Box<crate::types::TelegramPaymentPurposePremiumGiveaway>),
    /// The user buying Telegram Stars
    #[serde(rename(serialize = "telegramPaymentPurposeStars", deserialize = "telegramPaymentPurposeStars"))]
    Stars(Box<crate::types::TelegramPaymentPurposeStars>),
    /// The user buying Telegram Stars for other users
    #[serde(rename(serialize = "telegramPaymentPurposeGiftedStars", deserialize = "telegramPaymentPurposeGiftedStars"))]
    GiftedStars(Box<crate::types::TelegramPaymentPurposeGiftedStars>),
    /// The user creating a Telegram Star giveaway
    #[serde(rename(serialize = "telegramPaymentPurposeStarGiveaway", deserialize = "telegramPaymentPurposeStarGiveaway"))]
    StarGiveaway(Box<crate::types::TelegramPaymentPurposeStarGiveaway>),
    /// The user joins a chat and subscribes to regular payments in Telegram Stars
    #[serde(rename(serialize = "telegramPaymentPurposeJoinChat", deserialize = "telegramPaymentPurposeJoinChat"))]
    JoinChat(Box<crate::types::TelegramPaymentPurposeJoinChat>),
}

impl TelegramPaymentPurpose {
    /// Convenience constructor to create a [`TelegramPaymentPurpose::PremiumGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift(val: crate::types::TelegramPaymentPurposePremiumGift) -> Self {
        Self::PremiumGift(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::PremiumGiftCodes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_codes(val: crate::types::TelegramPaymentPurposePremiumGiftCodes) -> Self {
        Self::PremiumGiftCodes(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::PremiumGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_giveaway(val: crate::types::TelegramPaymentPurposePremiumGiveaway) -> Self {
        Self::PremiumGiveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::Stars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stars(val: crate::types::TelegramPaymentPurposeStars) -> Self {
        Self::Stars(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::GiftedStars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gifted_stars(val: crate::types::TelegramPaymentPurposeGiftedStars) -> Self {
        Self::GiftedStars(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::StarGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_giveaway(val: crate::types::TelegramPaymentPurposeStarGiveaway) -> Self {
        Self::StarGiveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`TelegramPaymentPurpose::JoinChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn join_chat(val: crate::types::TelegramPaymentPurposeJoinChat) -> Self {
        Self::JoinChat(Box::new(val))
    }

}

/// Converts a [`crate::types::TelegramPaymentPurposePremiumGift`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposePremiumGift> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposePremiumGift) -> Self {
        Self::PremiumGift(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposePremiumGiftCodes`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposePremiumGiftCodes> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposePremiumGiftCodes) -> Self {
        Self::PremiumGiftCodes(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposePremiumGiveaway`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposePremiumGiveaway> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposePremiumGiveaway) -> Self {
        Self::PremiumGiveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposeStars`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposeStars> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposeStars) -> Self {
        Self::Stars(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposeGiftedStars`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposeGiftedStars> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposeGiftedStars) -> Self {
        Self::GiftedStars(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposeStarGiveaway`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposeStarGiveaway> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposeStarGiveaway) -> Self {
        Self::StarGiveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::TelegramPaymentPurposeJoinChat`] into [`TelegramPaymentPurpose`].
impl From<crate::types::TelegramPaymentPurposeJoinChat> for TelegramPaymentPurpose {
    fn from(val: crate::types::TelegramPaymentPurposeJoinChat) -> Self {
        Self::JoinChat(Box::new(val))
    }
}

/// TDLib `ChatRevenueAmount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueAmount {
    /// Contains information about revenue earned from sponsored messages in a chat
    #[serde(rename(serialize = "chatRevenueAmount", deserialize = "chatRevenueAmount"))]
    ChatRevenueAmount(Box<crate::types::ChatRevenueAmount>),
}

impl ChatRevenueAmount {
    /// Convenience constructor to create a [`ChatRevenueAmount::ChatRevenueAmount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_revenue_amount(val: crate::types::ChatRevenueAmount) -> Self {
        Self::ChatRevenueAmount(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatRevenueAmount`] into [`ChatRevenueAmount`].
impl From<crate::types::ChatRevenueAmount> for ChatRevenueAmount {
    fn from(val: crate::types::ChatRevenueAmount) -> Self {
        Self::ChatRevenueAmount(Box::new(val))
    }
}

/// TDLib `ChatRevenueStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueStatistics {
    /// A detailed statistics about revenue earned from sponsored messages in a chat
    #[serde(rename(serialize = "chatRevenueStatistics", deserialize = "chatRevenueStatistics"))]
    ChatRevenueStatistics(Box<crate::types::ChatRevenueStatistics>),
}

impl ChatRevenueStatistics {
    /// Convenience constructor to create a [`ChatRevenueStatistics::ChatRevenueStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_revenue_statistics(val: crate::types::ChatRevenueStatistics) -> Self {
        Self::ChatRevenueStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatRevenueStatistics`] into [`ChatRevenueStatistics`].
impl From<crate::types::ChatRevenueStatistics> for ChatRevenueStatistics {
    fn from(val: crate::types::ChatRevenueStatistics) -> Self {
        Self::ChatRevenueStatistics(Box::new(val))
    }
}

/// Describes state of a revenue withdrawal
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RevenueWithdrawalState {
    /// Withdrawal is pending
    #[serde(rename(serialize = "revenueWithdrawalStatePending", deserialize = "revenueWithdrawalStatePending"))]
    Pending,
    /// Withdrawal succeeded
    #[serde(rename(serialize = "revenueWithdrawalStateSucceeded", deserialize = "revenueWithdrawalStateSucceeded"))]
    Succeeded(Box<crate::types::RevenueWithdrawalStateSucceeded>),
    /// Withdrawal failed
    #[serde(rename(serialize = "revenueWithdrawalStateFailed", deserialize = "revenueWithdrawalStateFailed"))]
    Failed,
}

impl RevenueWithdrawalState {
    /// Convenience constructor to create a [`RevenueWithdrawalState::Succeeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn succeeded(val: crate::types::RevenueWithdrawalStateSucceeded) -> Self {
        Self::Succeeded(Box::new(val))
    }

}

/// Converts a [`crate::types::RevenueWithdrawalStateSucceeded`] into [`RevenueWithdrawalState`].
impl From<crate::types::RevenueWithdrawalStateSucceeded> for RevenueWithdrawalState {
    fn from(val: crate::types::RevenueWithdrawalStateSucceeded) -> Self {
        Self::Succeeded(Box::new(val))
    }
}

/// Describes type of transaction for revenue earned from sponsored messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransactionType {
    /// Describes an unsupported transaction
    #[serde(rename(serialize = "chatRevenueTransactionTypeUnsupported", deserialize = "chatRevenueTransactionTypeUnsupported"))]
    Unsupported,
    /// Describes earnings from sponsored messages in a chat in some time frame
    #[serde(rename(serialize = "chatRevenueTransactionTypeSponsoredMessageEarnings", deserialize = "chatRevenueTransactionTypeSponsoredMessageEarnings"))]
    SponsoredMessageEarnings(Box<crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings>),
    /// Describes earnings from a published suggested post
    #[serde(rename(serialize = "chatRevenueTransactionTypeSuggestedPostEarnings", deserialize = "chatRevenueTransactionTypeSuggestedPostEarnings"))]
    SuggestedPostEarnings(Box<crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings>),
    /// Describes a withdrawal of earnings through Fragment
    #[serde(rename(serialize = "chatRevenueTransactionTypeFragmentWithdrawal", deserialize = "chatRevenueTransactionTypeFragmentWithdrawal"))]
    FragmentWithdrawal(Box<crate::types::ChatRevenueTransactionTypeFragmentWithdrawal>),
    /// Describes a refund for failed withdrawal of earnings through Fragment
    #[serde(rename(serialize = "chatRevenueTransactionTypeFragmentRefund", deserialize = "chatRevenueTransactionTypeFragmentRefund"))]
    FragmentRefund(Box<crate::types::ChatRevenueTransactionTypeFragmentRefund>),
}

impl ChatRevenueTransactionType {
    /// Convenience constructor to create a [`ChatRevenueTransactionType::SponsoredMessageEarnings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sponsored_message_earnings(val: crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings) -> Self {
        Self::SponsoredMessageEarnings(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatRevenueTransactionType::SuggestedPostEarnings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_post_earnings(val: crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings) -> Self {
        Self::SuggestedPostEarnings(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatRevenueTransactionType::FragmentWithdrawal`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment_withdrawal(val: crate::types::ChatRevenueTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatRevenueTransactionType::FragmentRefund`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment_refund(val: crate::types::ChatRevenueTransactionTypeFragmentRefund) -> Self {
        Self::FragmentRefund(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings`] into [`ChatRevenueTransactionType`].
impl From<crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings> for ChatRevenueTransactionType {
    fn from(val: crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings) -> Self {
        Self::SponsoredMessageEarnings(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings`] into [`ChatRevenueTransactionType`].
impl From<crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings> for ChatRevenueTransactionType {
    fn from(val: crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings) -> Self {
        Self::SuggestedPostEarnings(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatRevenueTransactionTypeFragmentWithdrawal`] into [`ChatRevenueTransactionType`].
impl From<crate::types::ChatRevenueTransactionTypeFragmentWithdrawal> for ChatRevenueTransactionType {
    fn from(val: crate::types::ChatRevenueTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatRevenueTransactionTypeFragmentRefund`] into [`ChatRevenueTransactionType`].
impl From<crate::types::ChatRevenueTransactionTypeFragmentRefund> for ChatRevenueTransactionType {
    fn from(val: crate::types::ChatRevenueTransactionTypeFragmentRefund) -> Self {
        Self::FragmentRefund(Box::new(val))
    }
}

/// TDLib `ChatRevenueTransaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransaction {
    /// Contains a chat revenue transactions
    #[serde(rename(serialize = "chatRevenueTransaction", deserialize = "chatRevenueTransaction"))]
    ChatRevenueTransaction(Box<crate::types::ChatRevenueTransaction>),
}

impl ChatRevenueTransaction {
    /// Convenience constructor to create a [`ChatRevenueTransaction::ChatRevenueTransaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_revenue_transaction(val: crate::types::ChatRevenueTransaction) -> Self {
        Self::ChatRevenueTransaction(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatRevenueTransaction`] into [`ChatRevenueTransaction`].
impl From<crate::types::ChatRevenueTransaction> for ChatRevenueTransaction {
    fn from(val: crate::types::ChatRevenueTransaction) -> Self {
        Self::ChatRevenueTransaction(Box::new(val))
    }
}

/// TDLib `ChatRevenueTransactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransactions {
    /// Contains a list of chat revenue transactions
    #[serde(rename(serialize = "chatRevenueTransactions", deserialize = "chatRevenueTransactions"))]
    ChatRevenueTransactions(Box<crate::types::ChatRevenueTransactions>),
}

impl ChatRevenueTransactions {
    /// Convenience constructor to create a [`ChatRevenueTransactions::ChatRevenueTransactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_revenue_transactions(val: crate::types::ChatRevenueTransactions) -> Self {
        Self::ChatRevenueTransactions(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatRevenueTransactions`] into [`ChatRevenueTransactions`].
impl From<crate::types::ChatRevenueTransactions> for ChatRevenueTransactions {
    fn from(val: crate::types::ChatRevenueTransactions) -> Self {
        Self::ChatRevenueTransactions(Box::new(val))
    }
}

/// TDLib `StarRevenueStatus` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarRevenueStatus {
    /// Contains information about Telegram Stars earned by a user or a chat
    #[serde(rename(serialize = "starRevenueStatus", deserialize = "starRevenueStatus"))]
    StarRevenueStatus(Box<crate::types::StarRevenueStatus>),
}

impl StarRevenueStatus {
    /// Convenience constructor to create a [`StarRevenueStatus::StarRevenueStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_revenue_status(val: crate::types::StarRevenueStatus) -> Self {
        Self::StarRevenueStatus(Box::new(val))
    }

}

/// Converts a [`crate::types::StarRevenueStatus`] into [`StarRevenueStatus`].
impl From<crate::types::StarRevenueStatus> for StarRevenueStatus {
    fn from(val: crate::types::StarRevenueStatus) -> Self {
        Self::StarRevenueStatus(Box::new(val))
    }
}

/// TDLib `StarRevenueStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarRevenueStatistics {
    /// A detailed statistics about Telegram Stars earned by a user or a chat
    #[serde(rename(serialize = "starRevenueStatistics", deserialize = "starRevenueStatistics"))]
    StarRevenueStatistics(Box<crate::types::StarRevenueStatistics>),
}

impl StarRevenueStatistics {
    /// Convenience constructor to create a [`StarRevenueStatistics::StarRevenueStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_revenue_statistics(val: crate::types::StarRevenueStatistics) -> Self {
        Self::StarRevenueStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::StarRevenueStatistics`] into [`StarRevenueStatistics`].
impl From<crate::types::StarRevenueStatistics> for StarRevenueStatistics {
    fn from(val: crate::types::StarRevenueStatistics) -> Self {
        Self::StarRevenueStatistics(Box::new(val))
    }
}

/// TDLib `GramRevenueStatus` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GramRevenueStatus {
    /// Contains information about TON Grams earned by the current user
    #[serde(rename(serialize = "gramRevenueStatus", deserialize = "gramRevenueStatus"))]
    GramRevenueStatus(Box<crate::types::GramRevenueStatus>),
}

impl GramRevenueStatus {
    /// Convenience constructor to create a [`GramRevenueStatus::GramRevenueStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gram_revenue_status(val: crate::types::GramRevenueStatus) -> Self {
        Self::GramRevenueStatus(Box::new(val))
    }

}

/// Converts a [`crate::types::GramRevenueStatus`] into [`GramRevenueStatus`].
impl From<crate::types::GramRevenueStatus> for GramRevenueStatus {
    fn from(val: crate::types::GramRevenueStatus) -> Self {
        Self::GramRevenueStatus(Box::new(val))
    }
}

/// TDLib `GramRevenueStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GramRevenueStatistics {
    /// A detailed statistics about TON Grams earned by the current user
    #[serde(rename(serialize = "gramRevenueStatistics", deserialize = "gramRevenueStatistics"))]
    GramRevenueStatistics(Box<crate::types::GramRevenueStatistics>),
}

impl GramRevenueStatistics {
    /// Convenience constructor to create a [`GramRevenueStatistics::GramRevenueStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gram_revenue_statistics(val: crate::types::GramRevenueStatistics) -> Self {
        Self::GramRevenueStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::GramRevenueStatistics`] into [`GramRevenueStatistics`].
impl From<crate::types::GramRevenueStatistics> for GramRevenueStatistics {
    fn from(val: crate::types::GramRevenueStatistics) -> Self {
        Self::GramRevenueStatistics(Box::new(val))
    }
}

