#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Options to be used when a Web App is opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppOpenParameters {
    /// Preferred Web App theme; pass null to use the default theme
    pub theme: Option<crate::types::ThemeParameters>,
    /// Short name of the current application; 0-64 English letters, digits, and underscores
    pub application_name: String,
    /// The mode in which the Web App is opened; pass null to open in webAppOpenModeFullSize
    pub mode: Option<crate::enums::WebAppOpenMode>,
}
