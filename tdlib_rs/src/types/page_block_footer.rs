#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The footer of a page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockFooter {
    /// Footer
    pub footer: crate::enums::RichText,
}
