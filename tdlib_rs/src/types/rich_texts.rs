#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A concatenation of rich texts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTexts {
    /// Texts
    pub texts: Vec<crate::enums::RichText>,
}
