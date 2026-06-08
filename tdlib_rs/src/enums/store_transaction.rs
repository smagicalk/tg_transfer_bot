#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoreTransaction {
    /// A purchase through App Store
    #[serde(rename(
        serialize = "storeTransactionAppStore",
        deserialize = "storeTransactionAppStore"
    ))]
    AppStore(crate::types::StoreTransactionAppStore),
    /// A purchase through Google Play
    #[serde(rename(
        serialize = "storeTransactionGooglePlay",
        deserialize = "storeTransactionGooglePlay"
    ))]
    GooglePlay(crate::types::StoreTransactionGooglePlay),
}
