#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The default background has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultBackground {
    /// True, if default background for dark theme has changed
    pub for_dark_theme: bool,
    /// The new default background; may be null
    pub background: Option<crate::types::Background>,
}
