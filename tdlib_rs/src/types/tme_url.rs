#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a URL linking to an internal Telegram entity
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TmeUrl {
    /// URL
    pub url: String,
    /// Type of the URL
    pub r#type: crate::enums::TmeUrlType,
}
