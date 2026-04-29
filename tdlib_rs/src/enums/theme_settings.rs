#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThemeSettings {
    /// Describes theme settings
    #[serde(rename(serialize = "themeSettings", deserialize = "themeSettings"))]
    ThemeSettings(crate::types::ThemeSettings),
}
