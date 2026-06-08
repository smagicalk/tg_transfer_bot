#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A pull quote
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockPullQuote {
    /// Quote text
    pub text: crate::enums::RichText,
    /// Quote credit
    pub credit: crate::enums::RichText,
}
