#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

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
