#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrl {
    /// Represents a URL linking to an internal Telegram entity
    #[serde(rename(serialize = "tMeUrl", deserialize = "tMeUrl"))]
    TmeUrl(crate::types::TmeUrl),
}
