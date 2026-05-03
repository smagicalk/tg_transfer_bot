#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockVerticalAlignment {
    /// The content must be top-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentTop",
        deserialize = "pageBlockVerticalAlignmentTop"
    ))]
    Top,
    /// The content must be middle-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentMiddle",
        deserialize = "pageBlockVerticalAlignmentMiddle"
    ))]
    Middle,
    /// The content must be bottom-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentBottom",
        deserialize = "pageBlockVerticalAlignmentBottom"
    ))]
    Bottom,
}
