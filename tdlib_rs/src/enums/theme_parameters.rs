#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThemeParameters {
    /// Contains parameters of the application theme
    #[serde(rename(serialize = "themeParameters", deserialize = "themeParameters"))]
    ThemeParameters(crate::types::ThemeParameters),
}
