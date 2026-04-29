#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of options for gifting Telegram Premium to a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftPaymentOptions {
    /// The list of options sorted by Telegram Premium subscription duration
    pub options: Vec<crate::types::PremiumGiftPaymentOption>,
}
