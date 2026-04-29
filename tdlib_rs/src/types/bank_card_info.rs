#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about a bank card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BankCardInfo {
    /// Title of the bank card description
    pub title: String,
    /// Actions that can be done with the bank card number
    pub actions: Vec<crate::types::BankCardActionOpenUrl>,
}
