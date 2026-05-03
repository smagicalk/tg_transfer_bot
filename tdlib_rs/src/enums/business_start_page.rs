#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessStartPage {
    /// Describes settings for a business account start page
    #[serde(rename(serialize = "businessStartPage", deserialize = "businessStartPage"))]
    BusinessStartPage(crate::types::BusinessStartPage),
}
