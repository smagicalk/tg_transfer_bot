#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessOpeningHours {
    /// Describes opening hours of a business
    #[serde(rename(
        serialize = "businessOpeningHours",
        deserialize = "businessOpeningHours"
    ))]
    BusinessOpeningHours(crate::types::BusinessOpeningHours),
}
