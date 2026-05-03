#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A purchase through Google Play
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoreTransactionGooglePlay {
    /// Application package name
    pub package_name: String,
    /// Identifier of the purchased store product
    pub store_product_id: String,
    /// Google Play purchase token
    pub purchase_token: String,
}
