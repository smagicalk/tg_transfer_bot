#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrls {
    /// Contains a list of t.me URLs
    #[serde(rename(serialize = "tMeUrls", deserialize = "tMeUrls"))]
    TmeUrls(crate::types::TmeUrls),
}
