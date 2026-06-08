#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A giveaway was created for the chat. Use telegramPaymentPurposePremiumGiveaway, storePaymentPurposePremiumGiveaway, telegramPaymentPurposeStarGiveaway, or storePaymentPurposeStarGiveaway to create a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayCreated {
    /// Number of Telegram Stars that will be shared by winners of the giveaway; 0 for Telegram Premium giveaways
    pub star_count: i64,
}
