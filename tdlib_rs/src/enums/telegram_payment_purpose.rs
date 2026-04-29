#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TelegramPaymentPurpose {
    /// The user gifting Telegram Premium to another user
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGift", deserialize = "telegramPaymentPurposePremiumGift"))]
    PremiumGift(crate::types::TelegramPaymentPurposePremiumGift),
    /// The user boosting a chat by creating Telegram Premium gift codes for other users
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGiftCodes", deserialize = "telegramPaymentPurposePremiumGiftCodes"))]
    PremiumGiftCodes(crate::types::TelegramPaymentPurposePremiumGiftCodes),
    /// The user creating a Telegram Premium giveaway
    #[serde(rename(serialize = "telegramPaymentPurposePremiumGiveaway", deserialize = "telegramPaymentPurposePremiumGiveaway"))]
    PremiumGiveaway(crate::types::TelegramPaymentPurposePremiumGiveaway),
    /// The user buying Telegram Stars
    #[serde(rename(serialize = "telegramPaymentPurposeStars", deserialize = "telegramPaymentPurposeStars"))]
    Stars(crate::types::TelegramPaymentPurposeStars),
    /// The user buying Telegram Stars for other users
    #[serde(rename(serialize = "telegramPaymentPurposeGiftedStars", deserialize = "telegramPaymentPurposeGiftedStars"))]
    GiftedStars(crate::types::TelegramPaymentPurposeGiftedStars),
    /// The user creating a Telegram Star giveaway
    #[serde(rename(serialize = "telegramPaymentPurposeStarGiveaway", deserialize = "telegramPaymentPurposeStarGiveaway"))]
    StarGiveaway(crate::types::TelegramPaymentPurposeStarGiveaway),
    /// The user joins a chat and subscribes to regular payments in Telegram Stars
    #[serde(rename(serialize = "telegramPaymentPurposeJoinChat", deserialize = "telegramPaymentPurposeJoinChat"))]
    JoinChat(crate::types::TelegramPaymentPurposeJoinChat),
}
