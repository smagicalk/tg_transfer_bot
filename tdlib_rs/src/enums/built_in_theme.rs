#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BuiltInTheme {
    /// Classic light theme
    #[serde(rename(serialize = "builtInThemeClassic", deserialize = "builtInThemeClassic"))]
    Classic,
    /// Regular light theme
    #[serde(rename(serialize = "builtInThemeDay", deserialize = "builtInThemeDay"))]
    Day,
    /// Regular dark theme
    #[serde(rename(serialize = "builtInThemeNight", deserialize = "builtInThemeNight"))]
    Night,
    /// Tinted dark theme
    #[serde(rename(serialize = "builtInThemeTinted", deserialize = "builtInThemeTinted"))]
    Tinted,
    /// Arctic light theme
    #[serde(rename(serialize = "builtInThemeArctic", deserialize = "builtInThemeArctic"))]
    Arctic,
}
