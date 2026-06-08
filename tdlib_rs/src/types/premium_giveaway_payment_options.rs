#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiveawayPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::PremiumGiveawayPaymentOption>,
}
