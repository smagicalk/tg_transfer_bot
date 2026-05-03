#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A header
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockHeader {
    /// Header
    pub header: crate::enums::RichText,
}
