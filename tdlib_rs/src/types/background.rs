#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a chat background
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Background {
    /// Unique background identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// True, if this is one of default backgrounds
    pub is_default: bool,
    /// True, if the background is dark and is recommended to be used with dark theme
    pub is_dark: bool,
    /// Unique background name
    pub name: String,
    /// Document with the background; may be null. Null only for filled and chat theme backgrounds
    pub document: Option<crate::types::Document>,
    /// Type of the background
    pub r#type: crate::enums::BackgroundType,
}
