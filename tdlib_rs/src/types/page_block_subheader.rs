#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A subheader
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSubheader {
    /// Subheader
    pub subheader: crate::enums::RichText,
}
