#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The subtitle of a page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSubtitle {
    /// Subtitle
    pub subtitle: crate::enums::RichText,
}
