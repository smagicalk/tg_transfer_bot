#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a product that can be paid with invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductInfo {
    /// Product title
    pub title: String,
    /// Product description
    pub description: crate::types::FormattedText,
    /// Product photo; may be null
    pub photo: Option<crate::types::Photo>,
}
