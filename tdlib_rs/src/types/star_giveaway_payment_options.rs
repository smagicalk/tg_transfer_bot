#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of options for creating of Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::StarGiveawayPaymentOption>,
}
