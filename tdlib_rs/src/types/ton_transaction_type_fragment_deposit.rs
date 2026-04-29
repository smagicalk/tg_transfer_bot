#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a deposit of Toncoins from Fragment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeFragmentDeposit {
    /// True, if the transaction is a gift from another user
    pub is_gift: bool,
    /// The sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
