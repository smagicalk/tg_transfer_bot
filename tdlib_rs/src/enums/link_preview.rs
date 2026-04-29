#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreview {
    /// Describes a link preview
    #[serde(rename(serialize = "linkPreview", deserialize = "linkPreview"))]
    LinkPreview(crate::types::LinkPreview),
}
