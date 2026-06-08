#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A purchase through App Store
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoreTransactionAppStore {
    /// App Store receipt
    pub receipt: String,
}
