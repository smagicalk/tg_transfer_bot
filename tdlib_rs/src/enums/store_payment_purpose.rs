#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorePaymentPurpose {
    /// The user subscribing to Telegram Premium
    #[serde(rename(
        serialize = "storePaymentPurposePremiumSubscription",
        deserialize = "storePaymentPurposePremiumSubscription"
    ))]
    PremiumSubscription(crate::types::StorePaymentPurposePremiumSubscription),
    /// The user gifting Telegram Premium to another user
    #[serde(rename(
        serialize = "storePaymentPurposePremiumGift",
        deserialize = "storePaymentPurposePremiumGift"
    ))]
    PremiumGift(crate::types::StorePaymentPurposePremiumGift),
    /// The user boosting a chat by creating Telegram Premium gift codes for other users
    #[serde(rename(
        serialize = "storePaymentPurposePremiumGiftCodes",
        deserialize = "storePaymentPurposePremiumGiftCodes"
    ))]
    PremiumGiftCodes(crate::types::StorePaymentPurposePremiumGiftCodes),
    /// The user creating a Telegram Premium giveaway
    #[serde(rename(
        serialize = "storePaymentPurposePremiumGiveaway",
        deserialize = "storePaymentPurposePremiumGiveaway"
    ))]
    PremiumGiveaway(crate::types::StorePaymentPurposePremiumGiveaway),
    /// The user creating a Telegram Star giveaway
    #[serde(rename(
        serialize = "storePaymentPurposeStarGiveaway",
        deserialize = "storePaymentPurposeStarGiveaway"
    ))]
    StarGiveaway(crate::types::StorePaymentPurposeStarGiveaway),
    /// The user buying Telegram Stars
    #[serde(rename(
        serialize = "storePaymentPurposeStars",
        deserialize = "storePaymentPurposeStars"
    ))]
    Stars(crate::types::StorePaymentPurposeStars),
    /// The user buying Telegram Stars for other users
    #[serde(rename(
        serialize = "storePaymentPurposeGiftedStars",
        deserialize = "storePaymentPurposeGiftedStars"
    ))]
    GiftedStars(crate::types::StorePaymentPurposeGiftedStars),
}
