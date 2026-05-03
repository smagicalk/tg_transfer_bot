#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeTheme {
    /// Name of the theme
    pub theme_name: String,
}
