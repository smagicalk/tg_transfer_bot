#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBusinessStartPage {
    /// Describes settings for a business account start page to set
    #[serde(rename(serialize = "inputBusinessStartPage", deserialize = "inputBusinessStartPage"))]
    InputBusinessStartPage(crate::types::InputBusinessStartPage),
}
