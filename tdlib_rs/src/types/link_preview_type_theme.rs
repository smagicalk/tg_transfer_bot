#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeTheme {
    /// The list of files with theme description
    pub documents: Vec<crate::types::Document>,
    /// Settings for the cloud theme; may be null if unknown
    pub settings: Option<crate::types::ThemeSettings>,
}
