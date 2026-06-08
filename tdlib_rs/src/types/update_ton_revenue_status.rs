#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The Toncoin revenue earned by the current user has changed. If Toncoin transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTonRevenueStatus {
    /// New Toncoin revenue status
    pub status: crate::types::TonRevenueStatus,
}
