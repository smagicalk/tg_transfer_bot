#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftsForResale {
    /// Describes gifts available for resale
    #[serde(rename(serialize = "giftsForResale", deserialize = "giftsForResale"))]
    GiftsForResale(crate::types::GiftsForResale),
}
