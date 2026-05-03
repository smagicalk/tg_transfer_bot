#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The title of a page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTitle {
    /// Title
    pub title: crate::enums::RichText,
}
