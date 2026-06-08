#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUserDeposit {
    /// Identifier of the user who gifted Telegram Stars; 0 if the user was anonymous
    pub user_id: i64,
    /// The sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
