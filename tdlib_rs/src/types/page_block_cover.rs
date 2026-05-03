#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A page cover
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCover {
    /// Cover
    pub cover: crate::enums::PageBlock,
}
