#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's utility bill
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementUtilityBill {
    /// The utility bill to be saved
    pub utility_bill: crate::types::InputPersonalDocument,
}
