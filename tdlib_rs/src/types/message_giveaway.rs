#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// Number of users which will receive Telegram Premium subscription gift codes
    pub winner_count: i32,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
