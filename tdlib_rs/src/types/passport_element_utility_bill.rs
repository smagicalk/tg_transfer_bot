#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element containing the user's utility bill
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementUtilityBill {
    /// Utility bill
    pub utility_bill: crate::types::PersonalDocument,
}
